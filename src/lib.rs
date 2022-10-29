pub mod base64 {


    use bitbuffer::{BitReadBuffer, BitReadStream, BigEndian, BitError};
    
    #[warn(unreachable_code)]
    #[warn(unused_variables)]
    pub fn encode(to_encode: &str) -> String {

        let mapping_table: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/', '='];

        let buffer = BitReadBuffer::new(to_encode.as_bytes(), BigEndian);

        let mut stream = BitReadStream::new(buffer.clone());
        
        let mut i : usize = 1;

        let mut index_mod : usize = 0; // the first, second or third 6-bits of 24-bits group

        let mut buffer_result : Vec<char> = vec![];
        loop {
            let number_result : Result<u8, BitError> = stream.read_sized(6);
            match number_result {
                Ok(number) => {
                    let char_to_print : char = mapping_table.get(number as usize).unwrap().clone();
                    // println!("{} -> Number: {}, Char_To_Print: {}", i, number, char_to_print);
                    buffer_result.push(char_to_print);
                    i += 1;
                    if index_mod == 0 {index_mod = 2}
                    else {index_mod = (index_mod-1)%3 ;}
                },
                Err(e) => {
                    match e {
                        bitbuffer::BitError::NotEnoughData { requested, bits_left } => {
                            if bits_left == 0  {break;}
                            println!("Requested: {} - Bits left : {}", requested, bits_left);
                            let mut number : u8 = stream.read_sized(bits_left).unwrap();

                            println!("Number before check: 0b{:6b}, bits Left: {}", number, bits_left);
                            // 🚧 I have no idea what i'm doing but it works
                            number = match bits_left {
                                1 => number << 5,
                                2 => number << 4,
                                3 => number << 3,
                                4 => number << 2,
                                5 => number << 1,
                                6 => number << 0,
                                _ => unreachable!()
                            };

                            let char_to_print : char = mapping_table.get(number as usize).unwrap().clone();
                            // println!("{:?}", v);
                            println!("{} -> Number: {:6b}, Char_To_Print: {}", i, number, char_to_print);
                            buffer_result.push(char_to_print);

                            // 🚧 I have no idea what i'm doing but it works
                            index_mod =  (buffer.bit_len() ^ 0b1) % 3; 
                            println!("Index mod {}", index_mod);
                            println!("Actual Buffer size = {}", buffer.bit_len()%3);
                            if index_mod == 0 { // Add two padding
                                buffer_result.push('=');
                                buffer_result.push('=');
                            } else if index_mod == 1 || index_mod == 2{ // Add one padding
                                buffer_result.push('=');
                            }
                            break;
                        }
                        _ => {
                            break;
                        }
                    }
                    
                }
            }
        }

        return String::from_iter(buffer_result);
    }

}

#[cfg(test)]
mod tests {
    use super::base64::{encode};

    #[test]
    fn is_encoded_fn_valid_hello_world() {
        let to_test = "SGVsbG8gV29ybGQgIQ==";
        let encoded = encode("Hello World !");
        assert_eq!(encoded,  to_test);
    }

    #[test]
    fn is_encoded_fn_valid_a() {
        let to_test = "QQ==";
        let encoded = encode("A");
        assert_eq!(encoded,  to_test);
    }

    #[test]
    fn is_encoded_fn_valid_aaa() {
        let to_test = "QUFB";
        let encoded = encode("AAA");
        assert_eq!(encoded,  to_test);
    }

    #[test]
    fn is_encoded_fn_valid_aaaa() {
        let to_test = "QUFBQQ==";
        let encoded = encode("AAAA");
        assert_eq!(encoded,  to_test);
    }

    #[test]
    fn is_encoded_fn_valid_two_padding() {
        let to_test = "TG9yZW0gSXBzdW0gaXMgc2ltcGx5IGR1bW15IHRleHQgb2YgdGhlIHByaW50aW5nIGFuZCB0eXBlc2V0dGluZyBpbmR1c3RyeS4gTG9yZW0gSXBzdW0gaGFzIGJlZW4gdGhlIGluZHVzdHJ5J3Mgc3RhbmRhcmQgZHVtbXkgdGV4dCBldmVyIHNpbmNlIHRoZSAxNTAwcywgd2hlbiBhbiB1bmtub3duIHByaW50ZXIgdG9vayBhIGdhbGxleSBvZiB0eXBlIGFuZCBzY3JhbWJsZWQgaXQgdG8gbWFrZSBhIHR5cGUgc3BlY2ltZW4gYm9vay4gSXQgaGFzIHN1cnZpdmVkIG5vdCBvbmx5IGZpdmUgY2VudHVyaWVzLCBidXQgYWxzbyB0aGUgbGVhcCBpbnRvIGVsZWN0cm9uaWMgdHlwZXNldHRpbmcsIHJlbWFpbmluZyBlc3NlbnRpYWxseSB1bmNoYW5nZWQuIEl0IHdhcyBwb3B1bGFyaXNlZCBpbiB0aGUgMTk2MHMgd2l0aCB0aGUgcmVsZWFzZSBvZiBMZXRyYXNldCBzaGVldHMgY29udGFpbmluZyBMb3JlbSBJcHN1bSBwYXNzYWdlcywgYW5kIG1vcmUgcmVjZW50bHkgd2l0aCBkZXNrdG9wIHB1Ymxpc2hpbmcgc29mdHdhcmUgbGlrZSBBbGR1cyBQYWdlTWFrZXIgaW5jbHVkaW5nIHZlcnNpb25zIG9mIExvcmVtIElwc3VtLg==";
        let encoded = encode("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.");
        assert_eq!(encoded,  to_test);
    }

    #[test]
    fn is_encoded_fn_valid_one_padding() {
        let to_test = "TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4gTW9yYmkgYSB0ZWxsdXMgcmhvbmN1cywgdGluY2lkdW50IG1hc3NhIHNpdCBhbWV0LCBmaW5pYnVzIGVzdC4gTnVsbGEgbmlzbCBlc3QsIHBlbGxlbnRlc3F1ZSBhdWN0b3IgZXJhdCBhdCwgZWxlbWVudHVtIHZhcml1cyBuaXNpLiBJbiBxdWlzIHBoYXJldHJhIG1hdXJpcy4gRXRpYW0gcGVsbGVudGVzcXVlIG9yY2kgZXQgYW50ZSBoZW5kcmVyaXQsIHZlbCBtYXR0aXMgc2VtIHBoYXJldHJhLiBOYW0gZWdldCBoZW5kcmVyaXQgbmVxdWUuIEN1cmFiaXR1ciBtZXR1cyBhbnRlLCBlbGVpZmVuZCBhYyBwZWxsZW50ZXNxdWUgbm9uLCBjb25ndWUgc2l0IGFtZXQgbGlndWxhLiBOdWxsYSBldSB1cm5hIGVzdC4gUGVsbGVudGVzcXVlIGhhYml0YW50IG1vcmJpIHRyaXN0aXF1ZSBzZW5lY3R1cyBldCBuZXR1cyBldCBtYWxlc3VhZGEgZmFtZXMgYWMgdHVycGlzIGVnZXN0YXMuIFBoYXNlbGx1cyBuZWMgc29kYWxlcyBlcmF0LCB2aXRhZSB2ZW5lbmF0aXMgc2FwaWVuLiBNYXVyaXMgc2l0IGFtZXQgc2VtIG1pLiBOdW5jIGVnZXQgbGVjdHVzIHBoYXJldHJhLCBsdWN0dXMgbGlndWxhIHNpdCBhbWV0LCBpbXBlcmRpZXQgbWF1cmlzLiBOdWxsYW0gaW4ganVzdG8gbmVjIG9yY2kgaGVuZHJlcml0IGRpZ25pc3NpbS4gTWF1cmlzIHNjZWxlcmlzcXVlIGJsYW5kaXQgbmlzbCBhIHNhZ2l0dGlzLiBQcm9pbiB0ZW1wb3IgY29uZ3VlIG1pLCBuZWMgaW50ZXJkdW0gZXggY29udmFsbGlzIHNpdCBhbWV0LiAgICAgICAgUHJhZXNlbnQgc2l0IGFtZXQgc2NlbGVyaXNxdWUgaXBzdW0sIHF1aXMgZWdlc3RhcyBsYWN1cy4gQWVuZWFuIHF1aXMgYmxhbmRpdCB0ZWxsdXMsIGF0IGludGVyZHVtIGVyYXQuIEludGVnZXIgaW4gZmFjaWxpc2lzIHZlbGl0LiBRdWlzcXVlIGF0IGF1Z3VlIGFsaXF1ZXQsIGRpY3R1bSB0b3J0b3IgZXQsIHRpbmNpZHVudCB0b3J0b3IuIFF1aXNxdWUgcmhvbmN1cyB1cm5hIHNhZ2l0dGlzIGlwc3VtIGNvbnNlcXVhdCwgZXQgZ3JhdmlkYSBhcmN1IGNvbnNlY3RldHVyLiBBZW5lYW4gdmVsIHJ1dHJ1bSB0b3J0b3IsIHNpdCBhbWV0IG1vbGVzdGllIGxlY3R1cy4gQ3JhcyBwcmV0aXVtIGNvbmRpbWVudHVtIGVsZW1lbnR1bS4gUGVsbGVudGVzcXVlIGFjIGNvbnNlY3RldHVyIG51bmMsIG5vbiBjb25zZXF1YXQgbG9yZW0uIENyYXMgZXUgdm9sdXRwYXQgaXBzdW0sIHZlbCBkaWduaXNzaW0gbGFjdXMuIE51bGxhbSBvcmNpIGxpZ3VsYSwgcHVsdmluYXIgbm9uIGp1c3RvIG5lYywgZnJpbmdpbGxhIGFsaXF1YW0gbGliZXJvLiBPcmNpIHZhcml1cyBuYXRvcXVlIHBlbmF0aWJ1cyBldCBtYWduaXMgZGlzIHBhcnR1cmllbnQgbW9udGVzLCBuYXNjZXR1ciByaWRpY3VsdXMgbXVzLiBGdXNjZSB0ZWxsdXMgbmlzbCwgZWxlbWVudHVtIGlkIHZlc3RpYnVsdW0gZWdldCwgZmFjaWxpc2lzIHF1aXMgbGFjdXMuIERvbmVjIG1hZ25hIHJpc3VzLCBncmF2aWRhIG5vbiBzYXBpZW4gdml0YWUsIGNvbnNlcXVhdCBkYXBpYnVzIGVzdC4gUHJvaW4gaW4gZGlnbmlzc2ltIHR1cnBpcywgdml0YWUgZmluaWJ1cyBqdXN0by4gSW50ZWdlciBpbiB0aW5jaWR1bnQgdmVsaXQuIERvbmVjIGZhdWNpYnVzIG5pYmggYXQgdHJpc3RpcXVlIHJ1dHJ1bS4gVmVzdGlidWx1bSBhYyBtYXR0aXMgbmlzaS4gTnVsbGFtIG51bGxhIGRvbG9yLCBjb21tb2RvIGFsaXF1YW0gdGluY2lkdW50IG5vbiwgY29uc2VxdWF0IGRhcGlidXMgdHVycGlzLiBQaGFzZWxsdXMgYXQgc3VzY2lwaXQgZmVsaXMuIFBoYXNlbGx1cyBldCBtb2xsaXMgbWV0dXMuIEV0aWFtIG5lYyBsb2JvcnRpcyBleC4gQWVuZWFuIHNhcGllbiBkb2xvciwgb3JuYXJlIHV0IGZlbGlzIHNlZCwgcnV0cnVtIHZlaGljdWxhIGFyY3UuIEFlbmVhbiBjb25zZWN0ZXR1ciBsZW8gYWMgdm9sdXRwYXQgcGhhcmV0cmEuIEludGVnZXIgbm9uIHBvc3VlcmUgZGlhbS4gU3VzcGVuZGlzc2UgbmVjIGFsaXF1ZXQgdXJuYS4gQ3VyYWJpdHVyIGEgbGVjdHVzIG1ldHVzLiBTZWQgZGljdHVtIGltcGVyZGlldCBzYXBpZW4sIGlkIGludGVyZHVtIGVsaXQgdGluY2lkdW50IG5vbi4gUGVsbGVudGVzcXVlIHNvZGFsZXMgZXJhdCBhIGxvcmVtIHB1bHZpbmFyLCBpZCB2b2x1dHBhdCBzZW0gY29tbW9kby4gTmFtIGZhY2lsaXNpcyB2aXRhZSBqdXN0byBpZCBwb3J0YS4gTmFtIGEgbGFvcmVldCBtZXR1cy4gRG9uZWMgbWkgbnVsbGEsIHZhcml1cyBpbiB2b2x1dHBhdCBxdWlzLCBmcmluZ2lsbGEgdXQgbGVjdHVzLiBOdWxsYW0gbnVuYyB2ZWxpdCwgc29kYWxlcyBub24gdWx0cmljZXMgdmVsLCBzZW1wZXIgZXQgcHVydXMuIE1vcmJpIGJpYmVuZHVtIHZpdmVycmEgdmFyaXVzLiBNYWVjZW5hcyBmYXVjaWJ1cyBuZXF1ZSBuZWMgdGVsbHVzIGJsYW5kaXQgdmVzdGlidWx1bS4gTnVsbGEgZWxlaWZlbmQgdml0YWUgZXJhdCBxdWlzIGNvbnNlY3RldHVyLiBQcm9pbiBpbiBjb252YWxsaXMgdGVsbHVzLiBFdGlhbSB1dCBtaSBldCBtZXR1cyBibGFuZGl0IG9ybmFyZSBpZCBlZ2V0IHF1YW0uIFZpdmFtdXMgZWdldCBtYXVyaXMgaW4gdGVsbHVzIGludGVyZHVtIHRpbmNpZHVudCBldCBzZWQgbmlzaS5EdWlzIGF0IG51bmMgaW4gdHVycGlzIGhlbmRyZXJpdCB2ZWhpY3VsYS4gTnVsbGFtIHRyaXN0aXF1ZSBvcmNpIG5vbiBhcmN1IGRhcGlidXMgdGVtcG9yLiBPcmNpIHZhcml1cyBuYXRvcXVlIHBlbmF0aWJ1cyBldCBtYWduaXMgZGlzIHBhcnR1cmllbnQgbW9udGVzLCBuYXNjZXR1ciByaWRpY3VsdXMgbXVzLiBNYWVjZW5hcyBzZW1wZXIgbmlzbCBzaXQgYW1ldCBlbGl0IHBsYWNlcmF0LCB1dCBjb25ndWUgdXJuYSByaG9uY3VzLiBJbiB0dXJwaXMgcXVhbSwgYWxpcXVhbSBldCBuaWJoIHF1aXMsIHNvbGxpY2l0dWRpbiBmcmluZ2lsbGEgdXJuYS4gTnVsbGEgZmluaWJ1cyBtaSB0aW5jaWR1bnQgcmhvbmN1cyBpbnRlcmR1bS4gRnVzY2UgZXQgZmluaWJ1cyBtZXR1cy4gUHJhZXNlbnQgdGVtcHVzIGFyY3Ugbm9uIGFyY3UgbGFjaW5pYSwgc2l0IGFtZXQgdWxsYW1jb3JwZXIgYXVndWUgY29uc2VxdWF0LiBQcmFlc2VudCBjb21tb2RvIG9yY2kgZHVpLCBldCB2YXJpdXMgaXBzdW0gbWF0dGlzIG5lYy4gTnVuYyBjb25ndWUgZmVybWVudHVtIG51bGxhLCBxdWlzIHJ1dHJ1bSBlcm9zIHRpbmNpZHVudCBhYy4gTnVuYyBhIG5lcXVlIGluIGxlY3R1cyBwb3J0dGl0b3IgYmxhbmRpdCBub24gdXQgZHVpLiBDcmFzIHNvZGFsZXMgdGVsbHVzIG5lcXVlLCBhdCBjb252YWxsaXMgaXBzdW0gdWx0cmljZXMgZXQuIFZpdmFtdXMgYmliZW5kdW0gZXggZXJhdCwgc29sbGljaXR1ZGluIGltcGVyZGlldCByaXN1cyBjb25kaW1lbnR1bSBlZ2U=";
        let encoded = encode("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi a tellus rhoncus, tincidunt massa sit amet, finibus est. Nulla nisl est, pellentesque auctor erat at, elementum varius nisi. In quis pharetra mauris. Etiam pellentesque orci et ante hendrerit, vel mattis sem pharetra. Nam eget hendrerit neque. Curabitur metus ante, eleifend ac pellentesque non, congue sit amet ligula. Nulla eu urna est. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Phasellus nec sodales erat, vitae venenatis sapien. Mauris sit amet sem mi. Nunc eget lectus pharetra, luctus ligula sit amet, imperdiet mauris. Nullam in justo nec orci hendrerit dignissim. Mauris scelerisque blandit nisl a sagittis. Proin tempor congue mi, nec interdum ex convallis sit amet.        Praesent sit amet scelerisque ipsum, quis egestas lacus. Aenean quis blandit tellus, at interdum erat. Integer in facilisis velit. Quisque at augue aliquet, dictum tortor et, tincidunt tortor. Quisque rhoncus urna sagittis ipsum consequat, et gravida arcu consectetur. Aenean vel rutrum tortor, sit amet molestie lectus. Cras pretium condimentum elementum. Pellentesque ac consectetur nunc, non consequat lorem. Cras eu volutpat ipsum, vel dignissim lacus. Nullam orci ligula, pulvinar non justo nec, fringilla aliquam libero. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Fusce tellus nisl, elementum id vestibulum eget, facilisis quis lacus. Donec magna risus, gravida non sapien vitae, consequat dapibus est. Proin in dignissim turpis, vitae finibus justo. Integer in tincidunt velit. Donec faucibus nibh at tristique rutrum. Vestibulum ac mattis nisi. Nullam nulla dolor, commodo aliquam tincidunt non, consequat dapibus turpis. Phasellus at suscipit felis. Phasellus et mollis metus. Etiam nec lobortis ex. Aenean sapien dolor, ornare ut felis sed, rutrum vehicula arcu. Aenean consectetur leo ac volutpat pharetra. Integer non posuere diam. Suspendisse nec aliquet urna. Curabitur a lectus metus. Sed dictum imperdiet sapien, id interdum elit tincidunt non. Pellentesque sodales erat a lorem pulvinar, id volutpat sem commodo. Nam facilisis vitae justo id porta. Nam a laoreet metus. Donec mi nulla, varius in volutpat quis, fringilla ut lectus. Nullam nunc velit, sodales non ultrices vel, semper et purus. Morbi bibendum viverra varius. Maecenas faucibus neque nec tellus blandit vestibulum. Nulla eleifend vitae erat quis consectetur. Proin in convallis tellus. Etiam ut mi et metus blandit ornare id eget quam. Vivamus eget mauris in tellus interdum tincidunt et sed nisi.Duis at nunc in turpis hendrerit vehicula. Nullam tristique orci non arcu dapibus tempor. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Maecenas semper nisl sit amet elit placerat, ut congue urna rhoncus. In turpis quam, aliquam et nibh quis, sollicitudin fringilla urna. Nulla finibus mi tincidunt rhoncus interdum. Fusce et finibus metus. Praesent tempus arcu non arcu lacinia, sit amet ullamcorper augue consequat. Praesent commodo orci dui, et varius ipsum mattis nec. Nunc congue fermentum nulla, quis rutrum eros tincidunt ac. Nunc a neque in lectus porttitor blandit non ut dui. Cras sodales tellus neque, at convallis ipsum ultrices et. Vivamus bibendum ex erat, sollicitudin imperdiet risus condimentum ege");
        assert_eq!(encoded,  to_test);
    }

    #[test]
    fn is_encoded_fn_valid_no_padding() {
        let to_test = "TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4gTW9yYmkgYSB0ZWxsdXMgcmhvbmN1cywgdGluY2lkdW50IG1hc3NhIHNpdCBhbWV0LCBmaW5pYnVzIGVzdC4gTnVsbGEgbmlzbCBlc3QsIHBlbGxlbnRlc3F1ZSBhdWN0b3IgZXJhdCBhdCwgZWxlbWVudHVtIHZhcml1cyBuaXNpLiBJbiBxdWlzIHBoYXJldHJhIG1hdXJpcy4gRXRpYW0gcGVsbGVudGVzcXVlIG9yY2kgZXQgYW50ZSBoZW5kcmVyaXQsIHZlbCBtYXR0aXMgc2VtIHBoYXJldHJhLiBOYW0gZWdldCBoZW5kcmVyaXQgbmVxdWUuIEN1cmFiaXR1ciBtZXR1cyBhbnRlLCBlbGVpZmVuZCBhYyBwZWxsZW50ZXNxdWUgbm9uLCBjb25ndWUgc2l0IGFtZXQgbGlndWxhLiBOdWxsYSBldSB1cm5hIGVzdC4gUGVsbGVudGVzcXVlIGhhYml0YW50IG1vcmJpIHRyaXN0aXF1ZSBzZW5lY3R1cyBldCBuZXR1cyBldCBtYWxlc3VhZGEgZmFtZXMgYWMgdHVycGlzIGVnZXN0YXMuIFBoYXNlbGx1cyBuZWMgc29kYWxlcyBlcmF0LCB2aXRhZSB2ZW5lbmF0aXMgc2FwaWVuLiBNYXVyaXMgc2l0IGFtZXQgc2VtIG1pLiBOdW5jIGVnZXQgbGVjdHVzIHBoYXJldHJhLCBsdWN0dXMgbGlndWxhIHNpdCBhbWV0LCBpbXBlcmRpZXQgbWF1cmlzLiBOdWxsYW0gaW4ganVzdG8gbmVjIG9yY2kgaGVuZHJlcml0IGRpZ25pc3NpbS4gTWF1cmlzIHNjZWxlcmlzcXVlIGJsYW5kaXQgbmlzbCBhIHNhZ2l0dGlzLiBQcm9pbiB0ZW1wb3IgY29uZ3VlIG1pLCBuZWMgaW50ZXJkdW0gZXggY29udmFsbGlzIHNpdCBhbWV0LiAgICAgICAgUHJhZXNlbnQgc2l0IGFtZXQgc2NlbGVyaXNxdWUgaXBzdW0sIHF1aXMgZWdlc3RhcyBsYWN1cy4gQWVuZWFuIHF1aXMgYmxhbmRpdCB0ZWxsdXMsIGF0IGludGVyZHVtIGVyYXQuIEludGVnZXIgaW4gZmFjaWxpc2lzIHZlbGl0LiBRdWlzcXVlIGF0IGF1Z3VlIGFsaXF1ZXQsIGRpY3R1bSB0b3J0b3IgZXQsIHRpbmNpZHVudCB0b3J0b3IuIFF1aXNxdWUgcmhvbmN1cyB1cm5hIHNhZ2l0dGlzIGlwc3VtIGNvbnNlcXVhdCwgZXQgZ3JhdmlkYSBhcmN1IGNvbnNlY3RldHVyLiBBZW5lYW4gdmVsIHJ1dHJ1bSB0b3J0b3IsIHNpdCBhbWV0IG1vbGVzdGllIGxlY3R1cy4gQ3JhcyBwcmV0aXVtIGNvbmRpbWVudHVtIGVsZW1lbnR1bS4gUGVsbGVudGVzcXVlIGFjIGNvbnNlY3RldHVyIG51bmMsIG5vbiBjb25zZXF1YXQgbG9yZW0uIENyYXMgZXUgdm9sdXRwYXQgaXBzdW0sIHZlbCBkaWduaXNzaW0gbGFjdXMuIE51bGxhbSBvcmNpIGxpZ3VsYSwgcHVsdmluYXIgbm9uIGp1c3RvIG5lYywgZnJpbmdpbGxhIGFsaXF1YW0gbGliZXJvLiBPcmNpIHZhcml1cyBuYXRvcXVlIHBlbmF0aWJ1cyBldCBtYWduaXMgZGlzIHBhcnR1cmllbnQgbW9udGVzLCBuYXNjZXR1ciByaWRpY3VsdXMgbXVzLiBGdXNjZSB0ZWxsdXMgbmlzbCwgZWxlbWVudHVtIGlkIHZlc3RpYnVsdW0gZWdldCwgZmFjaWxpc2lzIHF1aXMgbGFjdXMuIERvbmVjIG1hZ25hIHJpc3VzLCBncmF2aWRhIG5vbiBzYXBpZW4gdml0YWUsIGNvbnNlcXVhdCBkYXBpYnVzIGVzdC4gUHJvaW4gaW4gZGlnbmlzc2ltIHR1cnBpcywgdml0YWUgZmluaWJ1cyBqdXN0by4gSW50ZWdlciBpbiB0aW5jaWR1bnQgdmVsaXQuIERvbmVjIGZhdWNpYnVzIG5pYmggYXQgdHJpc3RpcXVlIHJ1dHJ1bS4gVmVzdGlidWx1bSBhYyBtYXR0aXMgbmlzaS4gTnVsbGFtIG51bGxhIGRvbG9yLCBjb21tb2RvIGFsaXF1YW0gdGluY2lkdW50IG5vbiwgY29uc2VxdWF0IGRhcGlidXMgdHVycGlzLiBQaGFzZWxsdXMgYXQgc3VzY2lwaXQgZmVsaXMuIFBoYXNlbGx1cyBldCBtb2xsaXMgbWV0dXMuIEV0aWFtIG5lYyBsb2JvcnRpcyBleC4gQWVuZWFuIHNhcGllbiBkb2xvciwgb3JuYXJlIHV0IGZlbGlzIHNlZCwgcnV0cnVtIHZlaGljdWxhIGFyY3UuIEFlbmVhbiBjb25zZWN0ZXR1ciBsZW8gYWMgdm9sdXRwYXQgcGhhcmV0cmEuIEludGVnZXIgbm9uIHBvc3VlcmUgZGlhbS4gU3VzcGVuZGlzc2UgbmVjIGFsaXF1ZXQgdXJuYS4gQ3VyYWJpdHVyIGEgbGVjdHVzIG1ldHVzLiBTZWQgZGljdHVtIGltcGVyZGlldCBzYXBpZW4sIGlkIGludGVyZHVtIGVsaXQgdGluY2lkdW50IG5vbi4gUGVsbGVudGVzcXVlIHNvZGFsZXMgZXJhdCBhIGxvcmVtIHB1bHZpbmFyLCBpZCB2b2x1dHBhdCBzZW0gY29tbW9kby4gTmFtIGZhY2lsaXNpcyB2aXRhZSBqdXN0byBpZCBwb3J0YS4gTmFtIGEgbGFvcmVldCBtZXR1cy4gRG9uZWMgbWkgbnVsbGEsIHZhcml1cyBpbiB2b2x1dHBhdCBxdWlzLCBmcmluZ2lsbGEgdXQgbGVjdHVzLiBOdWxsYW0gbnVuYyB2ZWxpdCwgc29kYWxlcyBub24gdWx0cmljZXMgdmVsLCBzZW1wZXIgZXQgcHVydXMuIE1vcmJpIGJpYmVuZHVtIHZpdmVycmEgdmFyaXVzLiBNYWVjZW5hcyBmYXVjaWJ1cyBuZXF1ZSBuZWMgdGVsbHVzIGJsYW5kaXQgdmVzdGlidWx1bS4gTnVsbGEgZWxlaWZlbmQgdml0YWUgZXJhdCBxdWlzIGNvbnNlY3RldHVyLiBQcm9pbiBpbiBjb252YWxsaXMgdGVsbHVzLiBFdGlhbSB1dCBtaSBldCBtZXR1cyBibGFuZGl0IG9ybmFyZSBpZCBlZ2V0IHF1YW0uIFZpdmFtdXMgZWdldCBtYXVyaXMgaW4gdGVsbHVzIGludGVyZHVtIHRpbmNpZHVudCBldCBzZWQgbmlzaS5EdWlzIGF0IG51bmMgaW4gdHVycGlzIGhlbmRyZXJpdCB2ZWhpY3VsYS4gTnVsbGFtIHRyaXN0aXF1ZSBvcmNpIG5vbiBhcmN1IGRhcGlidXMgdGVtcG9yLiBPcmNpIHZhcml1cyBuYXRvcXVlIHBlbmF0aWJ1cyBldCBtYWduaXMgZGlzIHBhcnR1cmllbnQgbW9udGVzLCBuYXNjZXR1ciByaWRpY3VsdXMgbXVzLiBNYWVjZW5hcyBzZW1wZXIgbmlzbCBzaXQgYW1ldCBlbGl0IHBsYWNlcmF0LCB1dCBjb25ndWUgdXJuYSByaG9uY3VzLiBJbiB0dXJwaXMgcXVhbSwgYWxpcXVhbSBldCBuaWJoIHF1aXMsIHNvbGxpY2l0dWRpbiBmcmluZ2lsbGEgdXJuYS4gTnVsbGEgZmluaWJ1cyBtaSB0aW5jaWR1bnQgcmhvbmN1cyBpbnRlcmR1bS4gRnVzY2UgZXQgZmluaWJ1cyBtZXR1cy4gUHJhZXNlbnQgdGVtcHVzIGFyY3Ugbm9uIGFyY3UgbGFjaW5pYSwgc2l0IGFtZXQgdWxsYW1jb3JwZXIgYXVndWUgY29uc2VxdWF0LiBQcmFlc2VudCBjb21tb2RvIG9yY2kgZHVpLCBldCB2YXJpdXMgaXBzdW0gbWF0dGlzIG5lYy4gTnVuYyBjb25ndWUgZmVybWVudHVtIG51bGxhLCBxdWlzIHJ1dHJ1bSBlcm9zIHRpbmNpZHVudCBhYy4gTnVuYyBhIG5lcXVlIGluIGxlY3R1cyBwb3J0dGl0b3IgYmxhbmRpdCBub24gdXQgZHVpLiBDcmFzIHNvZGFsZXMgdGVsbHVzIG5lcXVlLCBhdCBjb252YWxsaXMgaXBzdW0gdWx0cmljZXMgZXQuIFZpdmFtdXMgYmliZW5kdW0gZXggZXJhdCwgc29sbGljaXR1ZGluIGltcGVyZGlldCByaXN1cyBjb25kaW1lbnR1bSBlZ2V0";
        let encoded = encode("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi a tellus rhoncus, tincidunt massa sit amet, finibus est. Nulla nisl est, pellentesque auctor erat at, elementum varius nisi. In quis pharetra mauris. Etiam pellentesque orci et ante hendrerit, vel mattis sem pharetra. Nam eget hendrerit neque. Curabitur metus ante, eleifend ac pellentesque non, congue sit amet ligula. Nulla eu urna est. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Phasellus nec sodales erat, vitae venenatis sapien. Mauris sit amet sem mi. Nunc eget lectus pharetra, luctus ligula sit amet, imperdiet mauris. Nullam in justo nec orci hendrerit dignissim. Mauris scelerisque blandit nisl a sagittis. Proin tempor congue mi, nec interdum ex convallis sit amet.        Praesent sit amet scelerisque ipsum, quis egestas lacus. Aenean quis blandit tellus, at interdum erat. Integer in facilisis velit. Quisque at augue aliquet, dictum tortor et, tincidunt tortor. Quisque rhoncus urna sagittis ipsum consequat, et gravida arcu consectetur. Aenean vel rutrum tortor, sit amet molestie lectus. Cras pretium condimentum elementum. Pellentesque ac consectetur nunc, non consequat lorem. Cras eu volutpat ipsum, vel dignissim lacus. Nullam orci ligula, pulvinar non justo nec, fringilla aliquam libero. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Fusce tellus nisl, elementum id vestibulum eget, facilisis quis lacus. Donec magna risus, gravida non sapien vitae, consequat dapibus est. Proin in dignissim turpis, vitae finibus justo. Integer in tincidunt velit. Donec faucibus nibh at tristique rutrum. Vestibulum ac mattis nisi. Nullam nulla dolor, commodo aliquam tincidunt non, consequat dapibus turpis. Phasellus at suscipit felis. Phasellus et mollis metus. Etiam nec lobortis ex. Aenean sapien dolor, ornare ut felis sed, rutrum vehicula arcu. Aenean consectetur leo ac volutpat pharetra. Integer non posuere diam. Suspendisse nec aliquet urna. Curabitur a lectus metus. Sed dictum imperdiet sapien, id interdum elit tincidunt non. Pellentesque sodales erat a lorem pulvinar, id volutpat sem commodo. Nam facilisis vitae justo id porta. Nam a laoreet metus. Donec mi nulla, varius in volutpat quis, fringilla ut lectus. Nullam nunc velit, sodales non ultrices vel, semper et purus. Morbi bibendum viverra varius. Maecenas faucibus neque nec tellus blandit vestibulum. Nulla eleifend vitae erat quis consectetur. Proin in convallis tellus. Etiam ut mi et metus blandit ornare id eget quam. Vivamus eget mauris in tellus interdum tincidunt et sed nisi.Duis at nunc in turpis hendrerit vehicula. Nullam tristique orci non arcu dapibus tempor. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Maecenas semper nisl sit amet elit placerat, ut congue urna rhoncus. In turpis quam, aliquam et nibh quis, sollicitudin fringilla urna. Nulla finibus mi tincidunt rhoncus interdum. Fusce et finibus metus. Praesent tempus arcu non arcu lacinia, sit amet ullamcorper augue consequat. Praesent commodo orci dui, et varius ipsum mattis nec. Nunc congue fermentum nulla, quis rutrum eros tincidunt ac. Nunc a neque in lectus porttitor blandit non ut dui. Cras sodales tellus neque, at convallis ipsum ultrices et. Vivamus bibendum ex erat, sollicitudin imperdiet risus condimentum eget");
        assert_eq!(encoded,  to_test);
    }
}