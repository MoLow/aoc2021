

enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GraterThan,
    LessThan,
    Equal,
}
// operator from number
impl From<u8> for Operator {
    fn from(num: u8) -> Self {
        match num {
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Minimum,
            3 => Operator::Maximum,
            5 => Operator::GraterThan,
            6 => Operator::LessThan,
            7 => Operator::Equal,
            _ => panic!("Invalid operator number"),
        }
    }
}


enum Packet {
    Literal {
        version: u8,
        size: u64,
        value: u64,
    },
    Operation {
        version: u8,
        size: u64,
        op: Operator,
        child_packets: Vec<Packet>,
    }
}

impl std::str::FromStr for Packet {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut bytes = (0..input.len()).step_by(2).map(|i| u8::from_str_radix(&input[i..i+2], 16).unwrap()).collect::<Vec<u8>>();

       return Ok(Packet::parse_packet(&mut bytes))
    }
}
impl Packet {
    fn collect_bits(bytes: &mut Vec<u8>, bits: i8) -> usize {
        let mut sum: usize = 0;
        let mut counter = bits;
        while counter > 0 {
            sum <<= 8;
            let shift = counter % 8;
            sum += match shift {
                0 => bytes.remove(0) as usize,
                _ => Packet::shift_left(bytes, shift as u8) as usize,
            };
            counter -= match shift { 0 => 8, _ => shift };
        }
        return sum;
    }

    fn shift_left(bytes: &mut Vec<u8>, shift: u8) -> u8 {
        let r = bytes[0] >> (8 - shift);
        for i in 0..bytes.len() {
            bytes[i] <<= shift;
            if i + 1 < bytes.len() {
                bytes[i] |= bytes[i+1] >> (8 - shift);
                bytes[i+1] &= (1 << (8 - shift)) - 1;
            }
        }
        
        return r;
    }

    fn parse_packet(bytes: &mut Vec<u8>) -> Packet {
        let version = Packet::collect_bits(bytes, 3) as u8;
        let type_id = Packet::collect_bits(bytes, 3) as u8;
        let mut collected_bits = 6;

        if type_id == 4 {
            let (value, size) = Packet::parse_literal(bytes);
            return Packet::Literal { version, value, size: size + collected_bits };
        }

        let mut child_packets = Vec::new();
        let length_type_id = Packet::collect_bits(bytes, 1);
        let collect = match length_type_id { 0 => 15, 1 => 11, _ => 0 };
        let max_collect  = Packet::collect_bits(bytes, collect);
        collected_bits += (collect + 1) as u64;
        let mut collected = 0;
        while collected < max_collect {
            let packet = Packet::parse_packet(bytes);
            collected +=  match length_type_id { 0 => packet.size(), 1 => 1, _ => 0 };
            collected_bits += packet.size() as u64;
            child_packets.push(packet);
        }

        
        return Packet:: Operation {
            version: version,
            size: collected_bits,
            op: type_id.into(),
            child_packets,
        };
    }

    fn parse_literal(payload: &mut Vec<u8>) -> (u64, u64) {
        let mut sum: u64 = 0;
        let mut size = 0;
        loop {
            let flag = Packet::collect_bits(payload, 1);
            sum <<= 4;
            size += 5;
            sum += Packet::collect_bits(payload, 4) as u64;
            if flag == 0 {
                break;
            }
        }

        return (sum, size);
    }

    fn size(&self) -> usize {
        return match self {
            Packet::Literal { size, .. } => *size,
            Packet::Operation { size, .. } => *size,
        } as usize;
    }

    fn sum_versions(&self) -> u64 {
        return match self {
            Packet::Literal { version, .. } => *version as u64,
            Packet::Operation { version, child_packets, .. } => *version as u64 + child_packets.iter().map(|p| p.sum_versions()).sum::<u64>(),
        };
    }

    fn eval_operator(&self) -> u64 {
        return match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operation { op, child_packets, .. } => {
                return match op {
                    Operator::Sum => child_packets.iter().map(|p| p.eval_operator()).sum::<u64>(),
                    Operator::Product => child_packets.iter().map(|p| p.eval_operator()).product::<u64>(),
                    Operator::Minimum => child_packets.iter().map(|p| p.eval_operator()).min().unwrap(),
                    Operator::Maximum => child_packets.iter().map(|p| p.eval_operator()).max().unwrap(),
                    Operator::GraterThan => { (child_packets[0].eval_operator() > child_packets[1].eval_operator()) as u64 },
                    Operator::LessThan => { (child_packets[0].eval_operator() < child_packets[1].eval_operator()) as u64 },
                    Operator::Equal => { (child_packets[0].eval_operator() == child_packets[1].eval_operator()) as u64 },
                };
            }
        };
    }
}

static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (u64, u64) { 
    let packet: Packet = INPUT.parse().unwrap();

    return (packet.sum_versions(), packet.eval_operator());
}