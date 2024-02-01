# tencent_im_rs
Tencent Chat RestAPI 

## Tencent UserSig Generate
https://www.tencentcloud.com/ko/document/product/1047/34385

```Rust

let sdk_app_id:u32 = 0;
let secret_key:String = String::from("");
let tencent_user_sig = TencentUserSig::(sdk_app_id, secret_key);

let user_id: String = String::from("");
let expire: i64 = 0;

let user_sig = tencent_user_sig.generate_user_sig(user_id, expire);
```