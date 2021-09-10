fn main() {
  let mut ret = nfq::Queue::open().unwrap();
  ret.bind(std::env::args().nth(1).unwrap().parse().unwrap()).unwrap();
  loop{
    let ret0 = ret.recv().ok();
    if let Some(mut ret0) = ret0{
      let ret1 = {
        let from_etherparse_slicedpacket_to_owned_network_informations = ||{
          let ret2 = ret0.get_payload();
          let ret2 = etherparse::SlicedPacket::from_ip(ret2).ok();
          let ret2 = match ret2{
            Some(ret2)=>{
              let ret3 = match ret2.ip{
                Some(ret2)=>match ret2{
                  etherparse::InternetSlice::Ipv4(ret3)=>Some(ret3.to_header()),
                  _=>None
                },
                _=>None,
              };
              let ret3 = match ret3{
                Some(ret3)=>match ret2.transport{
                  Some(ret4)=>match ret4{
                    etherparse::TransportSlice::Tcp(ret4)=>Some((ret3, ret4.to_header())),
                    _=>None
                  },
                  _=>None
                },
                _=>None
              };
              let ret3 = match ret3{
                Some(ret3)=>Some((ret3.0, ret3.1, ret2.payload)),
                _=>None,
              };
              ret3
            },
            _=>None
          };
          ret2
        };
        from_etherparse_slicedpacket_to_owned_network_informations()
      };
      if let Some(ret1) = ret1{
        let ret2 = {
          let split_http_payload_into_u8_header_and_body = ||{
            let ret3 = ret1.2;
            let ret3 = {
              let ret4 = ret3.iter().fold((false, Vec::<u8>::new(), Vec::<u8>::new(), Vec::<u8>::new()), |mut p, v|{
                const SEARCH : &[u8; 4] = b"\r\n\r\n";
                p.1.push(*v);
                if p.1.len() == SEARCH.len(){
                  if &[p.1[0], p.1[1], p.1[2], p.1[3]] == SEARCH{ p.0 = true; }
                  if p.0 == true { p.3.append(&mut p.1); }
                  else { p.2.append(&mut p.1); }
                }
                p
              });
              let ret4 = (ret4.2, ret4.3);
              ret4
            };
            ret3
          };
          split_http_payload_into_u8_header_and_body()
        };
        let ret2 = {
          let split_http_payload_into_string_header_and_body = ||{
            (String::from_utf8_lossy(&ret2.0), String::from_utf8_lossy(&ret2.1))
          };
          split_http_payload_into_string_header_and_body()
        };
        let ret2 = {
          let split_http_payload_into_each_string_header_and_body = ||{
            (ret2.0.split("\r\n").collect::<Vec::<_>>(), ret2.1.split("\r\n").collect::<Vec::<_>>())
          };
          split_http_payload_into_each_string_header_and_body()
        };
        let ret2 = {
          let determine_if_http_html_and_no_encoding_so_we_able_to_reverse_it = ||{
            let is_http = match ret2.0.iter().nth(0){
              Some(x)=> match x.to_lowercase().starts_with("http"){ true=>Some(()), false=>None },
              _=>None
            }.is_some();
            let ret3 = ret2.0.iter().fold((false, true), |mut p, v|{
              if v.to_lowercase().starts_with("content-type: text/html") { p.0 = true; }
              if v.to_lowercase().starts_with("content-encoding") { p.0 = false; }
              p
            });
            let ret3 = is_http && ret3.0 && ret3.1;
            let ret3 = (ret2, ret3);
            ret3
          };
          determine_if_http_html_and_no_encoding_so_we_able_to_reverse_it()
        };
        if ret2.1 {
          dbg!(&ret2.0.0);
          ret0.set_payload({
            let ret3 = etherparse::PacketBuilder::ipv4(ret1.0.source, ret1.0.destination, ret1.0.time_to_live);
            let ret3 = ret3.tcp(
              ret1.1.source_port,
              ret1.1.destination_port,
              ret1.1.sequence_number,
              ret1.1.window_size
            );
            let ret3 = if ret1.1.ns  { ret3.ns() } else { ret3 };
            let ret3 = if ret1.1.fin { ret3.fin() } else { ret3 };
            let ret3 = if ret1.1.syn { ret3.syn() } else { ret3 };
            let ret3 = if ret1.1.rst { ret3.rst() } else { ret3 };
            let ret3 = if ret1.1.psh { ret3.psh() } else { ret3 };
            let ret3 = if ret1.1.ack { ret3.ack(ret1.1.acknowledgment_number) } else { ret3 };
            let ret3 = if ret1.1.urg { ret3.urg(ret1.1.urgent_pointer) } else { ret3 };
            let ret3 = if ret1.1.ece { ret3.ece() } else { ret3 };
            let ret3 = if ret1.1.cwr { ret3.cwr() } else { ret3 };
            let mut ret3 = (ret3, vec![], {
              fn traverse(node: Option<ego_tree::NodeMut<scraper::Node>>) {
                match node {
                  None => return,
                  Some(mut n) => {
                    traverse(n.first_child());
                    traverse(n.next_sibling());
                    if let scraper::Node::Text(text) = n.value().clone(){
                      n.insert_before(scraper::Node::Text(scraper::node::Text{text: tendril::format_tendril!("{}", String::from_utf8_lossy(&text.text.as_bytes().iter().rev().map(|v| *v).collect::<Vec::<_>>()))}));
                      n.detach();
                    }
                  }
                }
              }
              let mut ret4 = scraper::Html::parse_document(&ret2.0.1.join("\r\n"));
              traverse(Some(ret4.tree.root_mut()));
              let html = ret4.root_element().html();
              let ret4 = format!("{}\r\n\r\n{}{}", ret2.0.0.join("\r\n"), html, ret4.errors.iter().fold(String::from(""), |mut p, v| format!("{}{}", p, v)));
              dbg!(&ret4);
              let ret4 = ret4.as_bytes().to_vec();
              ret4
            });
            ret3.0.write(&mut ret3.1, &ret3.2);
            let ret3 = ret3.1;
            ret3
          });
        }
      }
      ret.verdict(ret0);
    }
  }
}