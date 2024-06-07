use avfaudio2_sys::*;

#[allow(non_upper_case_globals)]
pub const nil: id = 0 as *mut _;

pub fn main() {
    println!("Hello AVFAudio");
    
    unsafe { 
        let session = AVAudioSession::sharedInstance();
        #[allow(unused)]
        let mut error: *mut NSError = ::std::ptr::null_mut();
        session.setCategory_error_(AVAudioSessionCategoryAmbient, error);

        if error != nil as *mut _ {
            let str = error.as_ref().unwrap().localizedDescription();
            println!("Error");
            dbg!(str);
        }
    }
}