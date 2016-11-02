#[cfg(feature = "std")]
pub mod ipmon {
    use std::fs::File;
    use std::io::Read;
    use std::fmt::Debug;
    use std::net::Ipv4Addr;
    use std::str::FromStr;
    use std::string::String;

    use std::io::Cursor;

    extern crate byteorder;

    use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

    #[derive(Debug)]
    pub enum LocatorError {
        InvalidAddr(String),
        InvalidPrefix,
        InvalidCidrFormat(String),
    }


    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[derive(Debug)]
    pub struct Locator {
        textData: Vec<u8>,
        indexData1: Vec<u32>,
        indexData2: Vec<i32>,
        indexData3: Vec<i32>,
        index: Vec<i32>
    }


    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[derive(Debug)]
    pub struct LocationInfo {
        Country: String,
        Region: String,
        City: String,
        Isp: String
    }


    //#[allow(exceeding_bitshifts)]
    impl Locator {
        pub fn init(file: String) -> Result<Locator, LocatorError> {
            let file = File::open("ip.dat").unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            Locator::new(&buffer)
        }
        pub fn new(data: &Vec<u8>) -> Result<Locator, LocatorError> {
            let mut rdr = Cursor::new(&data[0..4]);
            let textoff = rdr.read_u32::<BigEndian>().unwrap();

            let mut textData = Vec::new();
            for i in &data[(textoff - 1024) as usize..data.len() as usize] {
                textData.push(*i);
            }
            let mut index = vec![0;256];
            for i in 0..256 {
                let off = 4 + i * 4;
                let mut r = Cursor::new(&data[off..off + 4]);
                index[i] = r.read_u32::<LittleEndian>().unwrap() as i32;
            }

            let nidx = ((textoff - 4 - 1024 - 1024) / 8) as usize;
            let mut indexData1: Vec<u32> = vec![0;nidx];
            let mut indexData2 = vec![0;nidx];
            let mut indexData3 = vec![0;nidx];
            for i in 0..nidx {
                let mut off: i32 = (4 + 1024 + i * 8) as i32;
                let mut r = Cursor::new(&data[off as usize..(off + 5) as usize]);
                indexData1[i as usize] = r.read_u32::<BigEndian>().unwrap() as u32;
                indexData2[i as usize] = data[(off + 4) as usize] as i32 | (data[(off + 5) as usize] as i32) << 8 | (data[(off + 6) as usize] as i32) << 16;
                indexData3[i as usize] = (data[(off + 7) as usize]) as i32;
            }
            Ok(Locator {
                textData: textData,
                indexData1: indexData1,
                indexData2: indexData2,
                indexData3: indexData3,
                index: index
            })
        }

        fn Find(&self, ip: String) -> Result<LocationInfo, LocatorError> {
            let ip: Ipv4Addr = Ipv4Addr::from_str(&ip).unwrap();
            let ipu = ip.octets();
            let mut r = Cursor::new(&ipu);
            let uip = r.read_u32::<BigEndian>().unwrap() as u32;
            self.FindByUint(uip)
        }


        fn FindByUint(&self, ip: u32) -> Result<LocationInfo, LocatorError> {
            let mut end = self.indexData1.len() - 1;
            if ip >> 24 != 0xff {
                end = (self.index[((ip >> 24) + 1) as usize]) as usize;
            }
            let idx = self.findIndexOffset(ip, self.index[(ip >> 24) as usize], end as i32);
            let off = self.indexData2[idx as usize];
            let _text: Vec<u8> = self.textData[off as usize..(off + self.indexData3[idx as usize]) as usize].to_vec();
            //        let data_str = String::from_utf8(data.to_vec()).expect("invalid utf8");
            let text = match String::from_utf8(_text.to_vec()) {
                Ok(s) => s,
                Err(e) => ""
            };

            let fields: Vec<&str> = data_str.split("\t").collect();
            let fields_length: i32 = fields.len();
            match fields_length {
                4 => Ok(LocationInfo { Country: fields[0].to_string(), Region: fields[1].to_string(), City: fields[2].to_string(), Isp: "".to_string() }),
                5 => Ok(LocationInfo { Country: fields[0].to_string(), Region: fields[1].to_string(), City: fields[2].to_string(), Isp: fields[4].to_string() }),
                _ => println!("incorrect fields numbers")
            }
        }

        fn findIndexOffset(&self, ip: u32, start: i32, end: i32) -> i32 {
            let mut start = start;
            let mut end = end;
            loop {
                if start < end {
                    let mid = (start + end) / 2;
                    if ip > self.indexData1[mid as usize] {
                        start = mid + 1;
                    } else {
                        end = mid
                    }
                } else {
                    break;
                }
            }
            if self.indexData1[end as usize] >= ip {
                return end;
            }
            return start;
        }
    }
}


    //    fn main()
    //    {
    //        let mut file = File::open("ip.dat").unwrap();
    //        let mut buffer = Vec::new();
    //        file.read_to_end(&mut buffer).unwrap();
    //        let mut locator = Locator::new(&buffer).unwrap();
    //        let info = locator.Find("202.96.209.5".to_string());
    //        match locator.Find("202.96.209.5".to_string()) {
    //            Ok(loc) => println!("location-> country: {}, region: {}, city: {}, isp: {}", loc.Country, loc.Region, loc.City, loc.Isp),
    //            Err(e) => println!("error: {:?}", e)
    //        }
