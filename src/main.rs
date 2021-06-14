use pretty_assertions::{assert_eq, assert_ne};

fn main() {
    ||->Result<(), ()>{
        let thread_output = std::thread::spawn(||{
            let mut queue_output = nfq::Queue::open().unwrap();
            queue_output.set_queue_max_len(u16::MAX, u32::MAX);
            queue_output.bind(1).map_err(|v|());
            let mut https = std::collections::HashSet::<std::net::Ipv4Addr>::new();
            let interfaces = pnet::datalink::interfaces();
            let interface_name = std::env::args().nth(1).unwrap();
            let interface = interfaces.into_iter().filter(|v|v.name==interface_name).next().unwrap();
            let eth_local = [0x92, 0x2a, 0x8d, 0x40, 0x26, 0x73]; 
            /*let eth_remote = [0xd2, 0x0f, 0x5b, 0x15, 0xb2, 0xad];*/ let eth_remote = [0x9E, 0x2D, 0x0B, 0x8A, 0xA9, 0x07];
            let (mut tx, mut rx) = match pnet::datalink::channel(&interface, pnet::datalink::Config::default()) {
                Ok(pnet::datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Unhandled channel type"),
                Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
            };
            loop {
                // rx.next().and_then(|rx|{
                //     dbg!();
                //     rx.iter().for_each(|v|{
                //         print!("{:02X}", v);
                //     });
                //     println!();
                //     Ok(())
                // });
                queue_output.recv().map_err(|v|())
                .and_then(|mut msg|{
                    let msg_payload = msg.get_payload().iter().cloned().collect::<Vec::<u8>>();
                    let ip = etherparse::SlicedPacket::from_ip(&msg_payload).map_err(|v|()).ok();
                    let ip_payload = ip.clone().and_then(|ip|{
                        std::str::from_utf8(ip.payload).map_err(|v|()).ok()
                    });
                    let match_http = (
                        ip_payload
                        .and_then(|str_payload|{
                            str_payload.split("\n").nth(0)
                            .and_then(|line0|{
                                line0.split_whitespace().last()
                            })
                            .and_then(|last_line0|{
                                if last_line0.starts_with("HTTP") { Some(true) }
                                else { None }
                            })
                        })
                        .is_some()
                    );
                    if match_http {
                        let ret = (
                            Some(())
                            .and_then(|_|{
                                Some(())
                                .and_then(|_|Some(({
                                    let ret = (
                                        [
                                            "HTTP/1.1 200 OK",
                                            "Accept-Ranges: bytes",
                                            "Content-Length: 2",
                                            "Connection: close",
                                            "Content-Type: text/html",
                                            "",
                                            "OK"
                                        ]
                                        .iter()
                                        .fold(Vec::<u8>::new(), |mut p, v|{
                                            v.as_bytes().iter().for_each(|v| p.push(*v));
                                            "\r\n".as_bytes().iter().for_each(|v| p.push(*v));
                                            p
                                        })
                                    );
                                    let ret = ret.iter().take(ret.len()-2).cloned().collect::<Vec::<u8>>();
                                    ret
                                },)))
                                // .and_then(|_|Some((
                                //     [0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x20, 0x32, 0x30, 0x30, 0x20, 0x4f, 0x4b, 0x0d, 0x0a, 0x44, 0x61, 0x74, 0x65, 0x3a, 0x20, 0x53, 0x75, 0x6e, 0x2c, 0x20, 0x33, 0x31, 0x20, 0x4a, 0x61, 0x6e, 0x20, 0x32, 0x30, 0x32, 0x31, 0x20, 0x30, 0x33, 0x3a, 0x35, 0x36, 0x3a, 0x34, 0x38, 0x20, 0x47, 0x4d, 0x54, 0x0d, 0x0a, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x3a, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x0d, 0x0a, 0x4c, 0x61, 0x73, 0x74, 0x2d, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x3a, 0x20, 0x57, 0x65, 0x64, 0x2c, 0x20, 0x30, 0x35, 0x20, 0x46, 0x65, 0x62, 0x20, 0x32, 0x30, 0x31, 0x34, 0x20, 0x31, 0x36, 0x3a, 0x30, 0x30, 0x3a, 0x33, 0x31, 0x20, 0x47, 0x4d, 0x54, 0x0d, 0x0a, 0x45, 0x54, 0x61, 0x67, 0x3a, 0x20, 0x22, 0x32, 0x38, 0x36, 0x2d, 0x34, 0x66, 0x31, 0x61, 0x61, 0x64, 0x62, 0x33, 0x31, 0x30, 0x35, 0x63, 0x30, 0x22, 0x0d, 0x0a, 0x41, 0x63, 0x63, 0x65, 0x70, 0x74, 0x2d, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x3a, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x0d, 0x0a, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x2d, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x3a, 0x20, 0x36, 0x34, 0x36, 0x0d, 0x0a, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x0d, 0x0a, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x2d, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x20, 0x74, 0x65, 0x78, 0x74, 0x2f, 0x68, 0x74, 0x6d, 0x6c, 0x0d, 0x0a, 0x0d, 0x0a, 0x3c, 0x68, 0x74, 0x6d, 0x6c, 0x3e, 0x3c, 0x68, 0x65, 0x61, 0x64, 0x3e, 0x3c, 0x2f, 0x68, 0x65, 0x61, 0x64, 0x3e, 0x3c, 0x62, 0x6f, 0x64, 0x79, 0x3e, 0x3c, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x3e, 0x0a, 0x3c, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3e, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x63, 0x65, 0x72, 0x6e, 0x2e, 0x63, 0x68, 0x3c, 0x2f, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3e, 0x0a, 0x3c, 0x2f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x3e, 0x0a, 0x0a, 0x3c, 0x68, 0x31, 0x3e, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x63, 0x65, 0x72, 0x6e, 0x2e, 0x63, 0x68, 0x20, 0x2d, 0x20, 0x68, 0x6f, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x77, 0x65, 0x62, 0x73, 0x69, 0x74, 0x65, 0x3c, 0x2f, 0x68, 0x31, 0x3e, 0x0a, 0x3c, 0x70, 0x3e, 0x46, 0x72, 0x6f, 0x6d, 0x20, 0x68, 0x65, 0x72, 0x65, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x63, 0x61, 0x6e, 0x3a, 0x3c, 0x2f, 0x70, 0x3e, 0x0a, 0x3c, 0x75, 0x6c, 0x3e, 0x0a, 0x3c, 0x6c, 0x69, 0x3e, 0x3c, 0x61, 0x20, 0x68, 0x72, 0x65, 0x66, 0x3d, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x63, 0x65, 0x72, 0x6e, 0x2e, 0x63, 0x68, 0x2f, 0x68, 0x79, 0x70, 0x65, 0x72, 0x74, 0x65, 0x78, 0x74, 0x2f, 0x57, 0x57, 0x57, 0x2f, 0x54, 0x68, 0x65, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x68, 0x74, 0x6d, 0x6c, 0x22, 0x3e, 0x42, 0x72, 0x6f, 0x77, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x77, 0x65, 0x62, 0x73, 0x69, 0x74, 0x65, 0x3c, 0x2f, 0x61, 0x3e, 0x3c, 0x2f, 0x6c, 0x69, 0x3e, 0x0a, 0x3c, 0x6c, 0x69, 0x3e, 0x3c, 0x61, 0x20, 0x68, 0x72, 0x65, 0x66, 0x3d, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x6c, 0x69, 0x6e, 0x65, 0x2d, 0x6d, 0x6f, 0x64, 0x65, 0x2e, 0x63, 0x65, 0x72, 0x6e, 0x2e, 0x63, 0x68, 0x2f, 0x77, 0x77, 0x77, 0x2f, 0x68, 0x79, 0x70, 0x65, 0x72, 0x74, 0x65, 0x78, 0x74, 0x2f, 0x57, 0x57, 0x57, 0x2f, 0x54, 0x68, 0x65, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x68, 0x74, 0x6d, 0x6c, 0x22, 0x3e, 0x42, 0x72, 0x6f, 0x77, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x77, 0x65, 0x62, 0x73, 0x69, 0x74, 0x65, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x2d, 0x6d, 0x6f, 0x64, 0x65, 0x20, 0x62, 0x72, 0x6f, 0x77, 0x73, 0x65, 0x72, 0x20, 0x73, 0x69, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x6f, 0x72, 0x3c, 0x2f, 0x61, 0x3e, 0x3c, 0x2f, 0x6c, 0x69, 0x3e, 0x0a, 0x3c, 0x6c, 0x69, 0x3e, 0x3c, 0x61, 0x20, 0x68, 0x72, 0x65, 0x66, 0x3d, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x68, 0x6f, 0x6d, 0x65, 0x2e, 0x77, 0x65, 0x62, 0x2e, 0x63, 0x65, 0x72, 0x6e, 0x2e, 0x63, 0x68, 0x2f, 0x74, 0x6f, 0x70, 0x69, 0x63, 0x73, 0x2f, 0x62, 0x69, 0x72, 0x74, 0x68, 0x2d, 0x77, 0x65, 0x62, 0x22, 0x3e, 0x4c, 0x65, 0x61, 0x72, 0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x69, 0x72, 0x74, 0x68, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x65, 0x62, 0x3c, 0x2f, 0x61, 0x3e, 0x3c, 0x2f, 0x6c, 0x69, 0x3e, 0x0a, 0x3c, 0x6c, 0x69, 0x3e, 0x3c, 0x61, 0x20, 0x68, 0x72, 0x65, 0x66, 0x3d, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x68, 0x6f, 0x6d, 0x65, 0x2e, 0x77, 0x65, 0x62, 0x2e, 0x63, 0x65, 0x72, 0x6e, 0x2e, 0x63, 0x68, 0x2f, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x22, 0x3e, 0x4c, 0x65, 0x61, 0x72, 0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x43, 0x45, 0x52, 0x4e, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x68, 0x79, 0x73, 0x69, 0x63, 0x73, 0x20, 0x6c, 0x61, 0x62, 0x6f, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x65, 0x62, 0x20, 0x77, 0x61, 0x73, 0x20, 0x62, 0x6f, 0x72, 0x6e, 0x3c, 0x2f, 0x61, 0x3e, 0x3c, 0x2f, 0x6c, 0x69, 0x3e, 0x0a, 0x3c, 0x2f, 0x75, 0x6c, 0x3e, 0x0a, 0x3c, 0x2f, 0x62, 0x6f, 0x64, 0x79, 0x3e, 0x3c, 0x2f, 0x68, 0x74, 0x6d, 0x6c, 0x3e, 0x0a]
                                // ,)))
                                .and_then(|ret|{
                                    etherparse::PacketHeaders::from_ip_slice(&msg_payload).map_err(|v|{panic!(); ()}).ok()
                                    .and_then(|ethernet|
                                        Some(())
                                        .and_then(|ethernet_head|{
                                            ethernet.ip.clone()
                                            .ok_or_else(||{panic!(); ()}).ok()
                                            .and_then(|ethernet_detail|
                                                if let etherparse::IpHeader::Version4(ip4) = ethernet_detail { Some(ip4) }
                                                else { None }
                                            )
                                            .and_then(|ethernet_detail| Some((ethernet_detail,)))
                                        })
                                        .and_then(|ethernet_head|{
                                            ethernet.transport.clone()
                                            .ok_or_else(||{panic!(); ()}).ok()
                                            .and_then(|ethernet_detail|
                                                if let etherparse::TransportHeader::Tcp(tcp) = ethernet_detail { Some(tcp) }
                                                else { None }
                                            )
                                            .and_then(|ethernet_detail| Some((ethernet_head.0, ethernet_detail,)))
                                        })
                                        .and_then(|ethernet_head|{
                                            etherparse::PacketHeaders::from_ethernet_slice(&ethernet.payload.clone())
                                            .ok().ok_or_else(||{panic!(); ()}).ok()
                                            .and_then(|prev_packet|
                                                Some(prev_packet.payload.to_vec())
                                            )
                                            .and_then(|prev_packet_payload|
                                                Some((ethernet_head.0, ethernet_head.1, prev_packet_payload))
                                            )
                                        })
                                        .and_then(|ethernet_head|{
                                            Some((
                                                ethernet_head.1.clone().options().iter().take(4).cloned().collect::<Vec::<u8>>(), (
                                                    ethernet_head.1.options().iter().skip(4).take(4).cloned().collect::<Vec::<u8>>(),
                                                    ethernet_head.1.options().iter().skip(4).skip(4).take(4).cloned().collect::<Vec::<u8>>()
                                                )
                                            ))
                                            .and_then(|options|
                                                Some((
                                                    ethernet_head.0, ethernet_head.1, ethernet_head.2, options
                                                ))
                                            )
                                        })
                                    )
                                    .and_then(|eth|Some((ret.0, eth)))
                                })
                            })
                            .and_then(|ethernet|{
                                let id : u16 = { use rand::Rng; rand::thread_rng().gen() };
                                let mut builder = |ethernet: (etherparse::Ipv4Header, etherparse::TcpHeader, Vec::<u8>, (std::vec::Vec<u8>, (std::vec::Vec<u8>, std::vec::Vec<u8>))), id: u16, seq_add: u32, function: fn(etherparse::PacketBuilderStep<etherparse::TcpHeader>)->Option::<etherparse::PacketBuilderStep<etherparse::TcpHeader>>, content: Vec::<u8>|{
                                    etherparse::PacketBuilder
                                    ::ethernet2(eth_remote, eth_local)
                                    .ip(etherparse::IpHeader::Version4({
                                        let mut ret = ethernet.0.clone();
                                        ret.differentiated_services_code_point = 10;
                                        ret.explicit_congestion_notification = 0;
                                        ret.identification = id;
                                        ret.source = ethernet.0.destination;
                                        ret.destination = ethernet.0.source;
                                        ret.time_to_live = 46;
                                        ret
                                    }))
                                    .tcp(
                                        ethernet.1.destination_port,
                                        ethernet.1.source_port,
                                        ethernet.1.acknowledgment_number+seq_add,
                                        ethernet.1.window_size
                                    )
                                    .ack(ethernet.1.sequence_number+{
                                        use std::convert::TryFrom;
                                        u32::try_from(ethernet.2.len()).unwrap()
                                    })
                                    .options_raw(&{
                                        let mut ret = Vec::<u8>::new();
                                        ret.extend(&ethernet.3.0);
                                        ret.extend(u32::to_ne_bytes(u32::from_ne_bytes(
                                            ethernet.3.1.1.iter().enumerate().fold([0;4], |mut p, v|{ p[v.0]=*v.1; p })
                                        )+100).to_vec());
                                        ret.extend(ethernet.3.1.0);
                                        ret
                                    })
                                    .ok()
                                    .ok_or_else(||{panic!(); ()}).ok()
                                    .and_then(function)
                                    .and_then(|builder|{
                                        let mut ret = Vec::<u8>::new();
                                        builder.write(&mut ret, &content);
                                        Some(ret)
                                    })
                                    .and_then(|packet|{
                                        let mut ret = Vec::<u8>::new();
                                        // ret.extend(&[
                                        //     0x00, 0x00, 0x00, 0x01, 0x00, 0x06, 0x9e, 0x2d, 0x0b, 0x8a, 0xa9, 0x07, 0x00, 0x00, 0x08, 0x00
                                        // ]);
                                        ret.extend(eth_local.to_vec());
                                        ret.extend(eth_remote.to_vec());
                                        ret.extend(vec![0x08, 0x00]);
                                        ret.extend(packet.iter().skip(14).cloned().collect::<Vec::<u8>>());
                                        // ret.extend(packet);
                                        Some(ret)
                                    })
                                    .and_then(|packet|{
                                        dbg!();
                                        packet.iter().for_each(|v|{
                                            print!("{:02X}", v);
                                        });
                                        println!();
                                        Some(packet)
                                    })
                                    .and_then(|packet|{
                                        tx.send_to(&packet, None)
                                    })
                                    .ok_or_else(||{panic!(); ()}).ok()
                                };
                                Some(())
                                .and_then(|_|{
                                    builder(ethernet.1.clone(), id+0, 0,
                                        |some| Some(some), 
                                        vec![]
                                    )
                                })
                                .and_then(|_|{
                                    builder(ethernet.1.clone(), id+1, 0,
                                        |some| Some(some.psh()), 
                                        ethernet.0.to_vec()
                                    )
                                })
                                .and_then(|_|{
                                    builder(ethernet.1.clone(), id+2, { use std::convert::TryFrom; u32::try_from(ethernet.0.len()).unwrap() },
                                        |some| Some(some.fin()),
                                        vec![]
                                    )
                                })
                            })
                        );
                        if let Some(_) = ret {
                            dbg!();
                            msg.set_verdict(nfq::Verdict::Drop);
                        }
                    } else { 
                        msg.set_verdict(nfq::Verdict::Accept); 
                    }
                    queue_output.verdict(msg);
                    Ok(())
                });
            }
        });
        std::thread::JoinHandle::join(thread_output).unwrap();
        Ok(())
    }();
}