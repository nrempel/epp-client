//! `EppObject` to EPP XML serialization tests

mod request {
    use super::super::get_xml;
    use super::super::CLTRID;
    use crate::common::HostAttrList;
    use crate::common::HostList;
    use crate::common::HostObjList;
    use crate::common::NoExtension;
    use crate::common::{
        Address, ContactStatus, DomainAuthInfo, DomainContact, DomainStatus, EppObject, HostAddr,
        HostAttr, HostStatus, Phone, PostalInfo,
    };
    use crate::contact::check::ContactCheck;
    use crate::contact::create::ContactCreate;
    use crate::contact::delete::ContactDelete;
    use crate::contact::info::ContactInfo;
    use crate::contact::update::ContactUpdate;
    use crate::domain::check::DomainCheck;
    use crate::domain::create::DomainCreate;
    use crate::domain::delete::DomainDelete;
    use crate::domain::info::DomainInfo;
    use crate::domain::renew::DomainRenew;
    use crate::domain::transfer::DomainTransferRequest;
    use crate::domain::transfer::DomainTransferUpdate;
    use crate::domain::update::DomainAddRemove;
    use crate::domain::update::DomainChangeInfo;
    use crate::domain::update::DomainUpdate;
    use crate::extensions::consolidate;
    use crate::extensions::consolidate::GMonthDay;
    use crate::extensions::namestore::NameStore;
    use crate::extensions::rgp::report::RgpRestoreReport;
    use crate::extensions::rgp::request::RgpRestoreRequest;
    use crate::hello::Hello;
    use crate::host::check::HostCheck;
    use crate::host::create::HostCreate;
    use crate::host::delete::HostDelete;
    use crate::host::info::HostInfo;
    use crate::host::update::HostAddRemove;
    use crate::host::update::HostChangeInfo;
    use crate::host::update::HostUpdate;
    use crate::login::Login;
    use crate::logout::Logout;
    use crate::message::ack::MessageAck;
    use crate::message::poll::MessagePoll;
    use crate::request::Transaction;
    use crate::xml::EppXml;
    use chrono::{DateTime, NaiveDate};
    use std::str::FromStr;

    #[test]
    fn hello() {
        let xml = get_xml("request/hello.xml").unwrap();
        let serialized = EppObject::<Hello>::build(Hello).serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn login() {
        let ext_uris = Some(vec![
            "http://schema.ispapi.net/epp/xml/keyvalue-1.0".to_string()
        ]);

        let xml = get_xml("request/login.xml").unwrap();
        let object = Login::new("username", "password", ext_uris);
        let serialized = object.serialize_request(None, CLTRID).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn logout() {
        let xml = get_xml("request/logout.xml").unwrap();
        let object = Logout;
        let serialized = object.serialize_request(None, CLTRID).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_check() {
        let xml = get_xml("request/contact/check.xml").unwrap();
        let object = ContactCheck::new(&["eppdev-contact-1", "eppdev-contact-2"]);
        let serialized =
            <ContactCheck as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_create() {
        let xml = get_xml("request/contact/create.xml").unwrap();

        let street = &["58", "Orchid Road"];
        let address = Address::new(street, "Paris", "Paris", "392374", "FR".parse().unwrap());
        let postal_info = PostalInfo::new("int", "John Doe", "Acme Widgets", address);
        let mut voice = Phone::new("+33.47237942");
        voice.set_extension("123");
        let mut fax = Phone::new("+33.86698799");
        fax.set_extension("677");

        let mut object = ContactCreate::new(
            "eppdev-contact-3",
            "contact@eppdev.net",
            postal_info,
            voice,
            "eppdev-387323",
        );
        object.set_fax(fax);

        let serialized =
            <ContactCreate as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_info() {
        let xml = get_xml("request/contact/info.xml").unwrap();

        let object = ContactInfo::new("eppdev-contact-3", "eppdev-387323");

        let serialized =
            <ContactInfo as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_update() {
        let xml = get_xml("request/contact/update.xml").unwrap();

        let mut object = ContactUpdate::new("eppdev-contact-3");

        let street = &["58", "Orchid Road"];
        let address = Address::new(street, "Paris", "Paris", "392374", "FR".parse().unwrap());
        let postal_info = PostalInfo::new("loc", "John Doe", "Acme Widgets", address);
        let voice = Phone::new("+33.47237942");

        object.set_info("newemail@eppdev.net", postal_info, voice, "eppdev-387323");
        let add_statuses = vec![ContactStatus {
            status: "clientTransferProhibited".to_string(),
        }];
        object.add(add_statuses);
        let remove_statuses = vec![ContactStatus {
            status: "clientDeleteProhibited".to_string(),
        }];
        object.remove(remove_statuses);

        let serialized =
            <ContactUpdate as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_delete() {
        let xml = get_xml("request/contact/delete.xml").unwrap();

        let object = ContactDelete::new("eppdev-contact-3");

        let serialized =
            <ContactDelete as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_check() {
        let xml = get_xml("request/domain/check.xml").unwrap();

        let object = DomainCheck::new(vec!["eppdev.com", "eppdev.net"]);

        let serialized =
            <DomainCheck as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_create() {
        let xml = get_xml("request/domain/create.xml").unwrap();

        let contacts = vec![
            DomainContact {
                contact_type: "admin".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "tech".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
        ];

        let object = DomainCreate::new(
            "eppdev-1.com",
            1,
            None,
            Some("eppdev-contact-3"),
            "epP4uthd#v",
            Some(contacts),
        );

        let serialized =
            <DomainCreate as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_create_with_host_obj() {
        let xml = get_xml("request/domain/create_with_host_obj.xml").unwrap();

        let contacts = vec![
            DomainContact {
                contact_type: "admin".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "tech".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
        ];

        let ns = Some(HostList::HostObjList(HostObjList {
            hosts: vec!["ns1.test.com".into(), "ns2.test.com".into()],
        }));

        let object = DomainCreate::new(
            "eppdev-1.com",
            1,
            ns,
            Some("eppdev-contact-3"),
            "epP4uthd#v",
            Some(contacts),
        );

        let serialized =
            <DomainCreate as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_create_with_host_attr() {
        let xml = get_xml("request/domain/create_with_host_attr.xml").unwrap();

        let contacts = vec![
            DomainContact {
                contact_type: "admin".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "tech".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
        ];

        let host_attr = HostList::HostAttrList(HostAttrList {
            hosts: vec![
                HostAttr {
                    name: "ns1.eppdev-1.com".into(),
                    addresses: None,
                },
                HostAttr {
                    name: "ns2.eppdev-1.com".into(),
                    addresses: Some(vec![
                        HostAddr::new_v4("177.232.12.58"),
                        HostAddr::new_v6("2404:6800:4001:801::200e"),
                    ]),
                },
            ],
        });

        let object = DomainCreate::new(
            "eppdev-2.com",
            1,
            Some(host_attr),
            Some("eppdev-contact-3"),
            "epP4uthd#v",
            Some(contacts),
        );

        let serialized =
            <DomainCreate as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_info() {
        let xml = get_xml("request/domain/info.xml").unwrap();

        let object = DomainInfo::new("eppdev.com", Some("2fooBAR"));

        let serialized =
            <DomainInfo as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_update() {
        let xml = get_xml("request/domain/update.xml").unwrap();

        let mut object = DomainUpdate::new("eppdev.com");

        let add = DomainAddRemove {
            ns: None,
            contacts: None,
            statuses: Some(vec![DomainStatus {
                status: "clientDeleteProhibited".to_string(),
            }]),
        };

        let remove = DomainAddRemove {
            ns: None,
            contacts: Some(vec![DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-2".to_string(),
            }]),
            statuses: None,
        };

        let change_info = DomainChangeInfo {
            registrant: None,
            auth_info: Some(DomainAuthInfo::new("epP5uthd#v")),
        };

        object.add(add);
        object.remove(remove);
        object.info(change_info);

        let serialized =
            <DomainUpdate as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_delete() {
        let xml = get_xml("request/domain/delete.xml").unwrap();

        let object = DomainDelete::new("eppdev.com");

        let serialized =
            <DomainDelete as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_renew() {
        let xml = get_xml("request/domain/renew.xml").unwrap();

        let exp_date = NaiveDate::from_ymd(2022, 7, 23);
        let object = DomainRenew::new("eppdev.com", exp_date, 1);

        let serialized =
            <DomainRenew as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_transfer_request() {
        let xml = get_xml("request/domain/transfer_request.xml").unwrap();

        let object = DomainTransferRequest::new("testing.com", Some(1), "epP4uthd#v");

        let serialized = <DomainTransferRequest as Transaction<NoExtension>>::serialize_request(
            object, None, CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_transfer_approve() {
        let xml = get_xml("request/domain/transfer_approve.xml").unwrap();

        let object = DomainTransferUpdate::approve("testing.com");

        let serialized = <DomainTransferUpdate as Transaction<NoExtension>>::serialize_request(
            object, None, CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_transfer_reject() {
        let xml = get_xml("request/domain/transfer_reject.xml").unwrap();

        let object = DomainTransferUpdate::reject("testing.com");

        let serialized = <DomainTransferUpdate as Transaction<NoExtension>>::serialize_request(
            object, None, CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_transfer_cancel() {
        let xml = get_xml("request/domain/transfer_cancel.xml").unwrap();

        let object = DomainTransferUpdate::cancel("testing.com");

        let serialized = <DomainTransferUpdate as Transaction<NoExtension>>::serialize_request(
            object, None, CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_transfer_query() {
        let xml = get_xml("request/domain/transfer_query.xml").unwrap();

        let object = DomainTransferRequest::query("testing.com", "epP4uthd#v");

        let serialized = <DomainTransferRequest as Transaction<NoExtension>>::serialize_request(
            object, None, CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn host_check() {
        let xml = get_xml("request/host/check.xml").unwrap();

        let object = HostCheck::new(&["ns1.eppdev-1.com", "host1.eppdev-1.com"]);

        let serialized =
            <HostCheck as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn host_create() {
        let xml = get_xml("request/host/create.xml").unwrap();

        let addresses = vec![
            HostAddr::new("v4", "29.245.122.14"),
            HostAddr::new("v6", "2404:6800:4001:801::200e"),
        ];

        let object = HostCreate::new("host1.eppdev-1.com", addresses);

        let serialized =
            <HostCreate as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn host_info() {
        let xml = get_xml("request/host/info.xml").unwrap();

        let object = HostInfo::new("ns1.eppdev-1.com");

        let serialized =
            <HostInfo as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn host_update() {
        let xml = get_xml("request/host/update.xml").unwrap();

        let addr = vec![HostAddr::new("v6", "2404:6800:4001:801::200e")];

        let add = HostAddRemove {
            addresses: Some(addr),
            statuses: None,
        };

        let remove = HostAddRemove {
            addresses: None,
            statuses: Some(vec![HostStatus {
                status: "clientDeleteProhibited".to_string(),
            }]),
        };

        let mut object = HostUpdate::new("host1.eppdev-1.com");

        object.add(add);
        object.remove(remove);
        object.info(HostChangeInfo {
            name: "host2.eppdev-1.com".into(),
        });

        let serialized =
            <HostUpdate as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn host_delete() {
        let xml = get_xml("request/host/delete.xml").unwrap();

        let object = HostDelete::new("ns1.eppdev-1.com");

        let serialized =
            <HostDelete as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn message_poll() {
        let xml = get_xml("request/message/poll.xml").unwrap();

        let object = MessagePoll::default();

        let serialized = object.serialize_request(None, CLTRID).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn message_ack() {
        let xml = get_xml("request/message/ack.xml").unwrap();

        let object = MessageAck::new(12345);

        let serialized = object.serialize_request(None, CLTRID).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn rgp_restore_request() {
        let xml = get_xml("request/extensions/rgp_restore_request.xml").unwrap();

        let domain_restore_request = RgpRestoreRequest::new();

        let mut object = DomainUpdate::new("eppdev.com");

        let change_info = DomainChangeInfo {
            registrant: None,
            auth_info: None,
        };

        object.info(change_info);

        let serialized = <DomainUpdate as Transaction<RgpRestoreRequest>>::serialize_request(
            object,
            Some(domain_restore_request),
            CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn rgp_restore_report() {
        let xml = get_xml("request/extensions/rgp_restore_report.xml").unwrap();

        let pre_data =
            "Pre-delete registration data goes here. Both XML and free text are allowed.";
        let post_data =
            "Post-restore registration data goes here. Both XML and free text are allowed.";
        let deleted_at = DateTime::from_str("2021-07-10T22:00:00.0Z").unwrap();
        let restored_at = DateTime::from_str("2021-07-20T22:00:00.0Z").unwrap();
        let restore_reason = "Registrant error.";
        let statements = &[
            "This registrar has not restored the Registered Name in order to assume the rights to use or sell the Registered Name for itself or for any third party.",
            "The information in this report is true to best of this registrar's knowledge, and this registrar acknowledges that intentionally supplying false information in this report shall constitute an incurable material breach of the Registry-Registrar Agreement.",
        ];
        let other = "Supporting information goes here.";

        let domain_restore_report = RgpRestoreReport::new(
            pre_data,
            post_data,
            deleted_at,
            restored_at,
            restore_reason,
            statements,
            other,
        );

        let mut object = DomainUpdate::new("eppdev.com");
        object.info(DomainChangeInfo {
            registrant: None,
            auth_info: None,
        });

        let serialized = <DomainUpdate as Transaction<RgpRestoreReport>>::serialize_request(
            object,
            Some(domain_restore_report),
            CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn namestore() {
        let xml = get_xml("request/extensions/namestore.xml").unwrap();

        let namestore_ext = NameStore::new("com");

        let object = DomainCheck::new(vec!["example1.com", "example2.com", "example3.com"]);

        let serialized = <DomainCheck as Transaction<NameStore>>::serialize_request(
            object,
            Some(namestore_ext),
            CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn consolidate() {
        let xml = get_xml("request/extensions/consolidate.xml").unwrap();

        let exp = GMonthDay::new(5, 31, None).unwrap();

        let consolidate_ext = consolidate::Sync::new(exp);

        let mut object = DomainUpdate::new("eppdev.com");

        object.info(DomainChangeInfo {
            registrant: None,
            auth_info: None,
        });

        let serialized = <DomainUpdate as Transaction<consolidate::Sync>>::serialize_request(
            object,
            Some(consolidate_ext),
            CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }
}
