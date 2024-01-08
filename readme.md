p = big prime
g = generator
h = generator (2)
q = "prime order" i.e, what number where g^q mod p and h^q mod p both = 1 and q is smallest value > 0 that satisfies this

https://crypto.stackexchange.com/questions/99262/chaum-pedersen-protocol

## Generating Parameters
Shared parameters for the protocol are loaded in via the environment (or, a `.env` file in the same directory). A sample set of initial parameters is provided in the included `.env` file, however if you would like to generate fresh ones you can run
```bash
./zkp-auth generate .env # output file path
```


## Client
> Input: x (a number) password

Client generates params - more trustless than server side generation?


## scratchpad notes
// https://docs.rs/static-dh-ecdh/latest/static_dh_ecdh/constants/constant.DH_GROUP_5_EXPONENT_LENGTH.html
// RFC 3526 - https://www.rfc-editor.org/rfc/rfc3526#section-2
// static DH_GROUP_5_PRIME: &str = "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA237327FFFFFFFFFFFFFFFF";
// static DH_GROUP_5_GENERATOR: usize = 2;
// static DH_GROUP_5_EXPONENT_LENGTH: usize = 192;

/**
*      p: BigInt::from_str("42765216643065397982265462252423826320512529931694366715111734768493812630447").unwrap(),
       q: BigInt::from_str("21382608321532698991132731126211913160256264965847183357555867384246906315223").unwrap(),
       g: BigInt::from_str("4").unwrap(),
       h: BigInt::from_str("9").unwrap(),
*/

## Improvements
- Split up client/server into completely different packages for separate deployment (together now for ease of use / simplicity)
- Backend user session tracking - can only register once. Maybe allow a method for user to overwrite their y1/y2 params