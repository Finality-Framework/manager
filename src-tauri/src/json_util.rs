pub fn get_legal_json(json_text:&str){
  let mut key_buf:String = "".to_string();
  let mut token_stack:Vec<JsonToken> = Vec::new();
  let mut is_key_parsing = true;
  let mut is_string_parsing = false;
  let mut is_array_parsing = false;
  for c in json_text.to_string().chars(){
    if is_space(&c){
      continue;
    }
    if is_key_parsing && c == '\"'{
      continue;
    }
    match c {
        '\"' => {
        if is_string_parsing {
          is_string_parsing = false;
          token_stack.pop();
        }else {
            is_string_parsing = true;
            token_stack.push(JsonToken::Quote);
        }
      }
      _ => {}
    }
    if is_string_parsing{
      continue;
    }
    match c {
      ':' => {
        is_key_parsing = false;
      }
      '}' => {
          let top_token = token_stack.pop().unwrap();
          if top_token != JsonToken::Object{
            token_stack.push(top_token)
          }
      },
      '{' => {token_stack.push(JsonToken::Object)},
      ']' => {
          let top_token = token_stack.pop().unwrap();
          if top_token != JsonToken::Array{
            token_stack.push(top_token)
          }
          is_array_parsing = false;
      }
      '[' => {
        token_stack.push(JsonToken::Array);
        is_array_parsing = true;
      },
      _ =>  {
        key_buf = key_buf + &c.to_string();
      }
    }
  }
}

fn is_space(c:&char) -> bool{
  match c {
      '\u{0020}' => return true,
      '\u{000D}' => return true,
      '\u{000A}' => return true,
      '\u{0009}' => return true,
      _ => return false
  }
}

#[derive(PartialEq)]
enum JsonToken {
    Array,
    Object,
    Quote
}