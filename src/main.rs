pub fn hamming_distance(string1: &&[u8], string2: &&[u8]) -> usize {
    let mut distance = 0;
    let mut i = 0;
    
    while i < string1.len() {
        if string1[i] != string2[i] {
            distance += 1;
        }
        i += 1;
    }
    return distance;
}
use std::time::Instant; 
fn main() {
    let string1 = "geekspracticehguisdhgiiuweyriuwehiiuweyriuwehfwebfuwebguyuheruihgvuyebghuiiuweyriuwehfwebfuwebguyuheruihgvuyebghuyveuyrghfiuhwueyveuyrghfiuhwuefwebfuwebguyuheruiiuweyriuwehfwebfuwebguyuheruihgvuyebghuyveuyrghfiuhwueihgvuyebghuyveuyrghfiuhwueihfiiuweyriuwehfwebfuwebguyuheruihgvuyebghuyveuyrghfiuhwueiwuehuyfgbwuybfviyuwehbiufh";
    let string2 = "nerdspractiseeruhfiuerhgbgtreughiuewriiuweyriuwehfwebfuwebguyiiuweyriuwehfwebfuwebguyuheruihgvuyebghuyveuyrghfiuhwueuheruihgvuyebghuyveuyrghfiuhwuehfiuwiiuweyriuwehfwebfuwebguyuheruihgvuyebghuyveuyrghfiuhwueehfuibuyweiiuweyriuwehfwebfuwebguyuheruihgvuyebghuyveuyrghfiuhwuebiughneruijhfoweifjwoeijguibhniurteboehjoirwhjeoifhoou";
    let string1 = string1.as_bytes();
    let string2 = string2.as_bytes();
    let r1 = &string1;
    let r2 = &string2;
    let mut x = 0;
    let start = Instant::now();
    for _i in 0..10000000 {
        x = hamming_distance(r1, r2);
        // x = 0;
        // let mut i = 0;
        // while i < string1.len() {
        //     if string1[i] != string2[i] {
        //         x += 1;
        //     }
        //     i += 1;
        // }
        //return distance;
    }
    let duration = start.elapsed();
    println!("{}",x);
    println!("Time elapsed is: {:?}", duration);
}
