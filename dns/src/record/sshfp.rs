use log::*;

use crate::wire::*;


/// A **SSHFP** _(secure shell fingerprint)_ record, which contains the
/// fingerprint of an SSH public key.
///
/// # References
///
/// - [RFC 4255](https://tools.ietf.org/html/rfc4255) — Using DNS to Securely Publish Secure Shell (SSH) Key Fingerprints (January 2006)
#[derive(PartialEq, Debug)]
pub struct SSHFP {

    /// The algorithm of the public key. This is a number with several defined
    /// mappings.
    pub algorithm: u8,

    /// The type of the fingerprint, which specifies the hashing algorithm
    /// used to derive the fingerprint. This is a number with several defined
    /// mappings.
    pub fingerprint_type: u8,

    /// The fingerprint of the public key.
    pub fingerprint: Vec<u8>,
}

impl Wire for SSHFP {
    const NAME: &'static str = "SSHFP";
    const RR_TYPE: u16 = 44;

    #[cfg_attr(all(test, feature = "with_mutagen"), ::mutagen::mutate)]
    fn read(stated_length: u16, c: &mut Cursor<&[u8]>) -> Result<Self, WireError> {
        let algorithm = c.read_u8()?;
        trace!("Parsed algorithm -> {:?}", algorithm);

        let fingerprint_type = c.read_u8()?;
        trace!("Parsed fingerprint type -> {:?}", fingerprint_type);

        if stated_length <= 2 {
            panic!("Length too short");
        }

        let fingerprint_length = stated_length - 1 - 1;
        let mut fingerprint = Vec::new();
        for _ in 0 .. fingerprint_length {
            fingerprint.push(c.read_u8()?);
        }

        Ok(Self { algorithm, fingerprint_type, fingerprint })
    }
}

impl SSHFP {

    /// Returns the hexadecimal representation of the fingerprint.
    pub fn hex_fingerprint(&self) -> String {
        self.fingerprint.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses() {
        let buf = &[
            0x01,  // algorithm
            0x01,  // fingerprint type
            0x21, 0x22, 0x23, 0x24,  // an extremely short fingerprint
        ];

        assert_eq!(SSHFP::read(buf.len() as _, &mut Cursor::new(buf)).unwrap(),
                   SSHFP {
                       algorithm: 1,
                       fingerprint_type: 1,
                       fingerprint: vec![ 0x21, 0x22, 0x23, 0x24 ],
                   });
    }

    #[test]
    fn buffer_ends_abruptly() {
        let buf = &[
            0x01,  // algorithm
        ];

        assert_eq!(SSHFP::read(6, &mut Cursor::new(buf)),
                   Err(WireError::IO));
    }

    #[test]
    fn hex_rep() {
        let sshfp = SSHFP {
            algorithm: 1,
            fingerprint_type: 1,
            fingerprint: vec![ 0xf3, 0x48, 0xcd, 0xc9 ],
        };

        assert_eq!(sshfp.hex_fingerprint(),
                   String::from("f348cdc9"));
    }

}
