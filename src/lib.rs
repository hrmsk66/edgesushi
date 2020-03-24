use serde_derive::*;
#[derive(Debug,Serialize,Deserialize)]
pub struct Order<'a> {
    neta: &'a str,
    nums: u32,
}

pub fn take_order(orders: &[Order]) -> String {
  orders.into_iter()
    .filter(|order| order.neta == "tuna" || order.neta == "tea" || order.neta == "shrimp")
    .fold(String::new(), |mut s, order| {
      let neta = match order.neta {
        "tuna"   => "🍣 ",
        "tea"    => "🍵 ",
        "shrimp" => "🍤 ",
        _ => unreachable!()
      };
      (0..order.nums).for_each(|_| s.push_str(neta));
      s
    })
}

pub fn text() -> String {
    String::from(r#"
               _     _
              | |   (_)
 ___ _   _ ___| |__  _
/ __| | | / __| '_ \| |
\__ \ |_| \__ \ | | | |
|___/\__,_|___/_| |_|_|
"#)
}