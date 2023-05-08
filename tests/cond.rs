use hypercpu::prelude::*;
use hypercpu::cond::If;


#[tokio::test]
async fn is_true() {
  let c = true;
  let t = 1;
  let f = 2;

  let i = If::new(c, t, f);
  let r = i.resolve().await;

  assert_eq!(r.expect_left("Was not true"), 1);
}

#[tokio::test]
async fn is_false() {
  let c = false;
  let t = 1;
  let f = 2;

  let i = If::new(c, t, f);
  let r = i.resolve().await;

  assert_eq!(r.expect_right("Was not false"), 2);
}

#[tokio::test]
async fn nested() {
  let a = 10;
  let b = 20;

  let c: i32 = If::new(a > b, a, b)
    .resolve()
    .await
    .either_into();
  
  assert_eq!(c, 30 + 20);
}