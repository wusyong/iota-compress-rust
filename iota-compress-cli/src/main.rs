extern crate iota_compress;
extern crate rand;
extern crate time;

use iota_compress::compress;
use iota_compress::decompress;
use rand::Rng;
use time::PreciseTime;

const PACKET_SIZE: usize = 2673;

fn main() {
    // let trytes = "XABBUCUCZAABUCUCVA9BUAUAZAZAVAVAWAXAYASC9BSCVAXA999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999BTVMYILK9HKYJONEAIWYZRBWOILDZJVXBZORHEODILGMOXERZTPKZKITDYIADWTFNMTCGNXWKXBJMSDNW999999999999999999999999999BEXAOBNBYAXA999999999999999INKSSAD99999999999999999999KUAYHLBWRLAZVUPFSCQGHTZENAIB9ITBJKGEJYBRQT9QHGEORCIUIQNGXPXGBYQPZSFVTCKGL9DVZJZX9SDRZBQDU9A9ESNFRSVVMKIPKSAPCMNADAD9RYCUBGGXFLYG9QYFLAYPQOFNIUUPRVNRNGMHBLCBWA9999USHSZBKJCFG9NUWHDOAZAZYGWBSS9HWZWP9PXJCO9MCQYNPVSSYCKCDGBTBYXTMTMCVXZSECTMSYA9999YAXAOBNBYAXA999999999999999SPDDGRCNF999999999K99999999POWSRVIO9FREE9ALTPTGEVPNNNT";
    //let trytes = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
    //let trytes = "999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999";
    let num_iterations: u32 = 100;
    let mut total_compress: i64 = 0;
    let mut total_decompress: i64 = 0;
    let mut total_compressed_length: u32 = 0;

    for i in 0..num_iterations {
        let trytes = generate_random();

        let start_compress = PreciseTime::now();
        let compressed = compress(&trytes);
        let compress_duration = start_compress.to(PreciseTime::now()).num_nanoseconds().unwrap_or(0);

        let compressed_len = compressed.len() as u32;

        total_compress += compress_duration;
        total_compressed_length += compressed_len;

        let start_decompress = PreciseTime::now();
        decompress(compressed);
        let decompress_duration = start_decompress.to(PreciseTime::now()).num_nanoseconds().unwrap_or(0);

        total_decompress += decompress_duration;

        println!("{} Compress {} ns, Decompress {} ns, Compressed Size {}", i, compress_duration, decompress_duration, compressed_len);

        //println!("{:?} {}", compressed, compressed.len());

        //println!("{:?} {}", decompressed, decompressed.len());

        //println!("matches {}", decompressed == trytes);
    }

    println!("Average Compress {} ns, Average Decompress {} ns, Average Compressed Size {} bytes", 
      total_compress / i64::from(num_iterations),
      total_decompress / i64::from(num_iterations),
      total_compressed_length / num_iterations
    );
}

fn generate_random() -> String {
    let tryte_alphabet: Vec<char> = vec!['9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

    let mut output: String = String::new();

    for _i in 0..PACKET_SIZE {
        let num = rand::thread_rng().gen_range(0, 27);
        output.push(tryte_alphabet[num]);
    }

    output
}