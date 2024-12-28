# Rust 发送邮件模板

当前目录新建一个`.env`:

```js
//发送方邮箱地址
mine_email=

//smtp服务器地址
smtp_server=

//smtp密钥
smtp_password=
```

## 尝试发送邮件

网页模板文件: `templates/index.html`

- 编辑main.rs:

    ```js
    // 收件人邮箱
    email_receiver=""

    // 邮件标题
    email_title=""

    // 邮件内容模板变量覆盖
    mail_body
    ```
- 运行:
    ```bash
    cargo build;cargo run
    ```

log:

```log
PS C:\Users\14752\codes\rust_reactive_email_template> cargo build;cargo run
   Compiling stable_deref_trait v1.2.0
   Compiling litemap v0.7.4
   Compiling writeable v0.5.5
   Compiling icu_locid_transform_data v1.5.0
   Compiling icu_properties_data v1.5.0
   Compiling cfg-if v1.0.0
   Compiling icu_normalizer_data v1.5.0
   Compiling utf8_iter v1.0.4
   Compiling utf16_iter v1.0.5
   Compiling smallvec v1.13.2
   Compiling write16 v1.0.0
   Compiling windows_x86_64_msvc v0.52.6
   Compiling memchr v2.7.4
   Compiling mime v0.3.17
   Compiling percent-encoding v2.3.1
   Compiling once_cell v1.20.2
   Compiling zerocopy v0.7.35
   Compiling zerofrom v0.1.5
   Compiling psm v0.1.24
   Compiling libc v0.2.169
   Compiling windows-targets v0.52.6                                                                                                                                                                                
   Compiling allocator-api2 v0.2.21
   Compiling slab v0.4.9
   Compiling yoke v0.7.5                                                                                                                                                                                            
   Compiling windows-sys v0.59.0
   Compiling windows-core v0.52.0
   Compiling form_urlencoded v1.2.1
   Compiling windows-sys v0.52.0
   Compiling libm v0.2.11
   Compiling mime_guess v2.0.5                                                                                                                                                                                      
   Compiling futures-task v0.3.31
   Compiling futures-core v0.3.31
   Compiling base64 v0.22.1
   Compiling pin-utils v0.1.0
   Compiling futures-io v0.3.31
   Compiling minimal-lexical v0.2.1
   Compiling pin-project-lite v0.2.15                                                                                                                                                                               
   Compiling num-traits v0.2.19
   Compiling zerovec v0.10.4                                                                                                                                                                                        
   Compiling ahash v0.8.11
   Compiling fastrand v2.3.0
   Compiling askama_escape v0.10.3
   Compiling email_address v0.2.9
   Compiling quoted_printable v0.5.1
   Compiling nom v7.1.3
   Compiling futures-util v0.3.31
   Compiling httpdate v1.0.3                                                                                                                                                                                        
   Compiling dotenv v0.15.0
   Compiling email-encoding v0.3.1                                                                                                                                                                                  
   Compiling hashbrown v0.14.5                                                                                                                                                                                      
   Compiling humansize v2.1.3
   Compiling windows v0.52.0                                                                                                                                                                                        
   Compiling askama_derive v0.12.5                                                                                                                                                                                  
   Compiling socket2 v0.5.8
   Compiling tinystr v0.7.6
   Compiling icu_collections v1.5.0
   Compiling icu_locid v1.5.0
   Compiling hostname v0.4.0
   Compiling icu_provider v1.5.0
   Compiling askama v0.12.1
   Compiling icu_locid_transform v1.5.0
   Compiling icu_properties v1.5.1
   Compiling stacker v0.1.17
   Compiling schannel v0.1.27
   Compiling chumsky v0.9.3
   Compiling native-tls v0.2.12
   Compiling icu_normalizer v1.5.0
   Compiling idna_adapter v1.2.0
   Compiling idna v1.0.3
   Compiling url v2.5.4
   Compiling lettre v0.11.11
   Compiling rust_reactive_email_template v0.1.0 (C:\Users\14752\codes\rust_reactive_email_template)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.37s
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target\debug\rust_reactive_email_template.exe`
Email sent successfully!
PS C:\Users\14752\codes\rust_reactive_email_template>
```