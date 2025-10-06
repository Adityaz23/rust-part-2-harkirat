pub struct User{
   pub  name: String,
    pub age: u8,
    pub email: String,
    pub sign_in: bool,
}

pub fn create_user()->User{
    User{
        name: String::from("Aditya"),
        age:22,
        email:String::from("soniadityakumar651@mail.com"),
        sign_in:true,
    }
}

pub struct Rectangle{
    pub widht:i32,
    pub height:i32,
}

pub fn rect_len()-> Rectangle{
    Rectangle { widht: 22, height: -12 }
}

pub struct Square{
    pub w:i32, // mismatch types hai toh u32 karna pdega ya fir i32 dono ko.
    pub h:i32,
}

pub fn squ_len()-> Square{
    Square { w: 12, h: 12 }
}