use winapi::um::winuser::GetAsyncKeyState;

pub struct Key {
    press:bool,
    out:bool,
    key:i16
}
impl Key {
    pub unsafe fn check(&mut self) ->bool {
        if GetAsyncKeyState(self.key.into()) != 0 { //home key
            self.press = true;
        }
        else { //home key
            self.press = false;
            self.out = true;
        }
        //Essa lógica faz com que a ação não seja executada quando o botão continuar sendo pressionado
        if self.press && self.out {
            self.out = false;
            return true;
        }
        return false;
    }
    pub fn init(key_code: i16)-> Key {
        Key{press:false,out:true,key:key_code}
    }
}