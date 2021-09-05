use std::cmp::min;
use std::usize::MAX;
use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
  let s = stdin();
  let s = s.lock();
  let s: String = s.bytes()
    .map(|c| c.expect("failed reading char") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
  s.parse().ok().expect("failed parsing")
}

fn main() {
  let n: usize = read();
  let m: usize = read();
  let mut c = vec![0; m];
  for j in 0..m {
    c[j] = read();
  }
  c.sort();
  let mut dp = vec![MAX; n+1];
  dp[0] = 0;
  for i in 0..n {
    for j in 0..m {
      if i + c[j] > n {
        break;
      }
      dp[i+c[j]] = min(dp[i+c[j]], dp[i]+1);
    } 
  }
  println!("{}", dp[n]);
}