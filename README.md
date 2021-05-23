# A simple Redis Client

[Redis Protocol specification](https://redis.io/topics/protocol)

```rust
fn get(&mut self, key:&str) -> Result<String>{
        let query = format!("*2\r\n$3\r\nGET\r\n${}\r\n{}\r\n", key.len(), key);
        self.io.write(query.as_bytes())?;
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..])?;
        Ok(str::from_utf8(&buffer)?.to_string())
    }

    fn set(&mut self, key:&str, value: &str) -> Result<String>{
        let query = format!("*3\r\n$3\r\nSET\r\n${}\r\n{}\r\n${}\r\n{}\r\n", key.len(), key, value.len(), value);
        self.io.write(query.as_bytes())?;
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..])?;
        Ok(str::from_utf8(&buffer)?.to_string())
    }
```