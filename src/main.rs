pub fn hamming_distance(string1: &Vec<char>, string2: &Vec<char>) -> usize {
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
    let string1: Vec<char> = string1.chars().collect();
    let string2: Vec<char> = string2.chars().collect();
    let mut x = 0;
    let start = Instant::now();
    for _i in 0..10000000 {
        x = hamming_distance(&string1, &string2);
    }
    let duration = start.elapsed();
    println!("{}",x);
    println!("Time elapsed is: {:?}", duration);
}
