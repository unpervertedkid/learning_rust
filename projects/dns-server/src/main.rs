use std::net::Ipv4Addr;
use std::fs::File;
use std::io::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}

pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl BytePacketBuffer {
    /// Create a new BytePacketBuffer
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    /// Get the current position in the buffer
    fn pos(&self) -> usize {
        self.pos
    }

    /// Step the buffer forward a specified number of steps
    fn step(&mut self, steps: usize) -> Result<(), &'static str> {
        self.pos += steps;
        Ok(())
    }

    /// Seek to a specific position in the buffer
    fn seek(&mut self, pos: usize) -> Result<(), &'static str> {
        self.pos = pos;
        Ok(())
    }

    /// Read a single byte and move the position one step forward
    fn read(&mut self) -> Result<u8, &'static str> {
        if self.pos >= 512 {
            return Err("End of buffer");
        }
        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }

    /// Get a single byte, without changing the buffer position
    fn get(&self, pos: usize) -> Result<u8, &'static str> {
        if pos >= 512 {
            return Err("End of buffer");
        }
        Ok(self.buf[pos])
    }

    /// Get a range of bytes
    fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], &'static str> {
        if start + len >= 512 {
            return Err("End of buffer");
        }
        Ok(&self.buf[start..start + len])
    }

    /// Read two bytes, stepping two steps forward
    fn read_u16(&mut self) -> Result<u16, &'static str> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);

        Ok(res)
    }

    /// Read four bytes, stepping four steps forward
    fn read_u32(&mut self) -> Result<u32, &'static str> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | (self.read()? as u32);

        Ok(res)
    }

    /// Read a qname
    /// 
    /// The tricky part: REading domain names, taking labels into consideration
    /// Will take something like: [3]www[6]google[3]com[0] and append
    /// www.google.com to the outstring
    fn read_qname(&mut self, outstr: &mut String) -> Result<(), &'static str> {
        // Siince we might encounter jumps, we will keep track of our position
        // locally as opposed to using the position within the buffer
        // This allows us to move the shared position to a point past our current
        // qname, while keeping track of our progress on the current qname
        // using this variable
        let mut pos = self.pos();

        // Keep track of whether or not we have encountered a jump
        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;

        // Our delimiter which we append for each label. Since we dont want a 
        // dot at the beggining of the domain name, we will set it to an empty
        // for now and set it to "." at the end of the first iteration
        let mut delim = "";

        loop {
            // Dns Packets are untrusted data, so we need to be paranoid
            // Someone can craft a packet with a cycle in the jump instructions
            // This guards against such packets
            if jumps_performed > max_jumps {
                return Err("Limit of max jumps reached");
            }

            // At this point, we are at the beginning of a label
            // We need to read the length of the label to know how many bytes to read
            let len = self.get(pos)?;

            // Check if this is a jump by checking the two most significant bits
            // If the two most significant bits are set, then this is a jump
            // If not, then this is a length of a label
            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2)?;
                }

                // Read another byte, calculate offset and perform the jump by
                // updating our local position variable
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                // Update the jumped variable so we dont jump again
                jumped = true;
                jumps_performed += 1;

                continue;
            }
            // The base scenario, where wer're reading a label and appending it to the output
            else {
                // Move the position forward by one, since we've read the length
                pos += 1;

                // Domains are terminated by a length of 0
                // If we encounter that, we are done with the qname
                if len == 0 {
                    break;
                }

                // Append the delimiter to the output string
                outstr.push_str(delim);

                // Extract the actual ascii bytes of this label and append it to the output buffer
                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                // Update the delimiter to a dot, since we are moving to the next label
                delim = ".";

                // Move the position forward by the length of the label
                pos += len as usize;
            }
        }

        // If we jumped, then we are currently sitting at the end of the jump
        // instructions, so we need to move the position forward by one

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResultCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

impl ResultCode {
    pub fn from_num(num: u8) -> ResultCode {
        match num {
            1 => ResultCode::FORMERR,
            2 => ResultCode::SERVFAIL,
            3 => ResultCode::NXDOMAIN,
            4 => ResultCode::NOTIMP,
            5 => ResultCode::REFUSED,
            0 | _ => ResultCode::NOERROR,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16, // 16 bits

    pub recursion_desired: bool,    // 1 bit
    pub truncated_message: bool,    // 1 bit
    pub authoritative_answer: bool, // 1 bit
    pub opcode: u8,                 // 4 bits
    pub response: bool,             // 1 bit

    pub rescode: ResultCode,       // 4 bits
    pub checking_disabled: bool,   // 1 bit
    pub authed_data: bool,         // 1 bit
    pub z: bool,                   // 1 bit
    pub recursion_available: bool, // 1 bit

    pub questions: u16,             // 16 bits
    pub answers: u16,               // 16 bits
    pub authoritative_entries: u16, // 16 bits
    pub resource_entries: u16,      // 16 bits
}

impl DnsHeader {
    pub fn new() -> DnsHeader {
        DnsHeader {
            id: 0,

            recursion_desired: false,
            truncated_message: false,
            authoritative_answer: false,
            opcode: 0,
            response: false,

            rescode: ResultCode::NOERROR,
            checking_disabled: false,
            authed_data: false,
            z: false,
            recursion_available: false,

            questions: 0,
            answers: 0,
            authoritative_entries: 0,
            resource_entries: 0,
        }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), &'static str> {
        self.id = buffer.read_u16()?;

        let flags = buffer.read_u16()?;
        let a = (flags >> 8) as u8;
        let b = (flags & 0xFF) as u8;
        self.recursion_desired = (a & (1 << 0)) > 0;
        self.truncated_message = (a & (1 << 1)) > 0;
        self.authoritative_answer = (a & (1 << 2)) > 0;
        self.opcode = (a >> 3) & 0x0F;
        self.response = (a & (1 << 7)) > 0;

        self.rescode = ResultCode::from_num(b & 0x0F);
        self.checking_disabled = (b & (1 << 4)) > 0;
        self.authed_data = (b & (1 << 5)) > 0;
        self.z = (b & (1 << 6)) > 0;
        self.recursion_available = (b & (1 << 7)) > 0;

        self.questions = buffer.read_u16()?;
        self.answers = buffer.read_u16()?;
        self.authoritative_entries = buffer.read_u16()?;
        self.resource_entries = buffer.read_u16()?;

        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum QueryType {
    UNKNOWN(u16),
    A, // 1
}

impl QueryType {
    pub fn to_num(&self) -> u16 {
        match *self {
            QueryType::UNKNOWN(x) => x,
            QueryType::A => 1,
        }
    }

    pub fn from_num(num: u16) -> QueryType {
        match num {
            1 => QueryType::A,
            _ => QueryType::UNKNOWN(num),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: QueryType,
}

impl DnsQuestion {
    pub fn new(name: String, qtype: QueryType) -> DnsQuestion {
        DnsQuestion {
            name: name,
            qtype: qtype,
        }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), &'static str> {
        buffer.read_qname(&mut self.name)?;
        self.qtype = QueryType::from_num(buffer.read_u16()?); // qtype
        let _ = buffer.read_u16()?; // class

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum DnsRecord {
    UNKNOWN {
        domain: String,
        qtype: u16,
        data_len: u16,
        ttl: u32,
    }, // 0
    A {
        domain: String,
        addr: Ipv4Addr,
        ttl: u32,
    }, // 1
}

impl DnsRecord {
    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord, &'static str> {
        let mut domain = String::new();
        buffer.read_qname(&mut domain)?;

        let qtype_num = buffer.read_u16()?;
        let qtype = QueryType::from_num(qtype_num);
        let _ = buffer.read_u16()?;
        let ttl = buffer.read_u32()?;
        let data_len = buffer.read_u16()?;

        match qtype {
            QueryType::A => {
                let raw_addr = buffer.read_u32()?;
                let addr = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    ((raw_addr >> 0) & 0xFF) as u8,
                );

                Ok(DnsRecord::A {
                    domain: domain,
                    addr: addr,
                    ttl: ttl,
                })
            }
            QueryType::UNKNOWN(_) => {
                buffer.step(data_len as usize)?;

                Ok(DnsRecord::UNKNOWN {
                    domain: domain,
                    qtype: qtype_num,
                    data_len: data_len,
                    ttl: ttl,
                })
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn new() -> DnsPacket {
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            resources: Vec::new(),
        }
    }

    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<DnsPacket, &'static str> {
        let mut result = DnsPacket::new();
        result.header.read(buffer)?;

        for _ in 0..result.header.questions {
            let mut question = DnsQuestion::new("".to_string(), QueryType::UNKNOWN(0));
            question.read(buffer)?;
            result.questions.push(question);
        }

        for _ in 0..result.header.answers {
            let rec = DnsRecord::read(buffer)?;
            result.answers.push(rec);
        }
        for _ in 0..result.header.authoritative_entries {
            let rec = DnsRecord::read(buffer)?;
            result.authorities.push(rec);
        }
        for _ in 0..result.header.resource_entries {
            let rec = DnsRecord::read(buffer)?;
            result.resources.push(rec);
        }

        Ok(result)
    }
}

