use crate::adapters::transfer::{
    AddressData,
    ContactData,
    ContactTypeData,
    StaffData,
    StaffTypeData};
use crate::pb_staff::{Address, Contact, Staff,StaffType,ContactType};

impl From<StaffData> for Staff {
    fn from(value: StaffData) -> Self {
        let pb_address = value.address.iter().map(|add| {
            let pb_address = pb_staff::Address::from(add);

            return pb_address;
        }).collect();

        let pb_contacts = value.contacts.iter().map(|contact| {
            let pb_contact = pb_staff::Contact::from(contact);
            return pb_contact;
        }).collect();


        let staff = Self {
            id: value.id.to_string(),
            first_name: value.first_name,
            last_name: value.last_name,
            email_address: value.email_address,
            vehicle_registration: value.vehicle_registration,
            staff_type_id: value.staff_type_id.to_string(),
            staff_type: value.staff_type,
            contractor_id: value.tenant_id.to_string(),
            sex: value.sex,
            hourly_rate: value.hourly_rate,
            active: value.active,
            address: pb_address,
            contacts: pb_contacts,
            commence_date: None,
            operation_user_id: value.operation_user_id.to_string(),
        };

        return staff;
    }
}
impl From<Staff> for StaffData {
    fn from(value: Staff) -> Self {
        let data_address = value.address.iter().map(|add| {
            let pb_address = AddressData::from(add);
            return pb_address;
        }).collect();

        let data_contacts = value.contacts.iter().map(|contact| {
            let pb_contact = ContactData::from(contact);
            return pb_contact;
        }).collect();

        let id = Uuid::parse_str(value.id.as_str());
        let staff_id = id.unwrap_or_else(|_| Uuid::nil());

        let staff_type_identity = Uuid::parse_str(value.staff_type_id.as_str());
        let staff_type_identity = staff_type_identity.unwrap_or_else(|_| Uuid::nil());

        let staff = Self {
            id: staff_id,
            first_name: value.first_name,
            last_name: value.last_name,
            email_address: value.email_address,
            vehicle_registration: value.vehicle_registration,
            staff_type_id: staff_type_identity,
            staff_type: value.staff_type,
            tenant_id: Default::default(),
            sex: value.sex,
            hourly_rate: value.hourly_rate,
            active: value.active,
            commence_date: Default::default(),
            operation_user_id: Default::default(),
            address: data_address,
            contacts: data_contacts,
        };

        return staff;
    }
}
impl From<&AddressData> for pb_staff::Address {
    fn from(value: &AddressData) -> Self {
        return Self {
            id: value.id.to_string(),
            street_name: value.street_name.to_string(),
            suburb: value.suburb.to_string(),
            post_code: value.post_code.to_string(),
            state: value.state.to_string(),
            country: value.country.to_string(),
            primary: value.primary,
        };
    }
}
impl From<AddressData> for pb_staff::Address {
    fn from(value: AddressData) -> Self {
        return Self {
            id: value.id.to_string(),
            street_name: value.street_name.to_string(),
            suburb: value.suburb.to_string(),
            post_code: value.post_code.to_string(),
            state: value.state.to_string(),
            country: value.country.to_string(),
            primary: value.primary,
        };
    }
}
impl From<&Address> for AddressData {
    fn from(value: &Address) -> Self {
        let id = Uuid::parse_str(
            value.id.as_str());

        let address_id = id.unwrap_or_else(|_| Uuid::nil());

        return Self {
            id: address_id,
            street_name: value.street_name.to_string(),
            suburb: value.suburb.to_string(),
            post_code: value.post_code.to_string(),
            state: value.state.to_string(),
            country: value.country.to_string(),
            primary: value.primary,
        };
    }
}
impl From<Address> for AddressData {
    fn from(value: Address) -> Self {
        let id = Uuid::parse_str(
            value.id.as_str());

        let staff_id = id.unwrap_or_else(|_| uuid::Uuid::nil());

        return Self {
            id: staff_id,
            street_name: value.street_name.to_string(),
            suburb: value.suburb.to_string(),
            post_code: value.post_code.to_string(),
            state: value.state.to_string(),
            country: value.country.to_string(),
            primary: value.primary,
        };
    }
}
impl From<&ContactData> for pb_staff::Contact {
    fn from(value: &ContactData) -> Self {
        return Self {
            id: value.id.to_string(),
            contact_type_id: value.contact_type_id.to_string(),
            contact_type: value.contact_type.to_string(),
            contact: value.contact.to_string(),
            primary: value.primary,
        };
    }
}
impl From<&Contact> for ContactData {
    fn from(value: &Contact) -> Self {
        let id = Uuid::parse_str(value.id.as_str());
        let contact_id = id.unwrap_or_else(|_| uuid::Uuid::nil());

        let contact_type_id = Uuid::parse_str(value.contact_type_id.as_str());
        let contact_type_id = contact_type_id.unwrap_or_else(|_| uuid::Uuid::nil());

        return Self {
            id: contact_id,
            contact_type_id,
            contact_type: value.contact_type.to_string(),
            contact: value.contact.to_string(),
            primary: value.primary,
        };
    }
}
impl From<ContactData> for pb_staff::Contact {
    fn from(value: ContactData) -> Self {
        return Self {
            id: value.id.to_string(),
            contact_type_id: value.contact_type_id.to_string(),
            contact_type: value.contact_type.to_string(),
            contact: value.contact.to_string(),
            primary: value.primary,
        };
    }
}
impl From<Contact> for ContactData {
    fn from(value: Contact) -> Self {
        let id = Uuid::parse_str(value.id.as_str());
        let contact_id = id.unwrap_or_else(|_| uuid::Uuid::nil());

        let contact_type_id = Uuid::parse_str(value.contact_type_id.as_str());
        let contact_type_id = contact_type_id.unwrap_or_else(|_| uuid::Uuid::nil());

        return Self {
            id: contact_id,
            contact_type_id,
            contact_type: value.contact_type.to_string(),
            contact: value.contact.to_string(),
            primary: value.primary,
        };
    }
}
impl From<&StaffTypeData> for StaffType {
    fn from(value: &StaffTypeData) -> Self {
        return Self {
            id: value.id.to_string(),
            staff_type: value.staff_type.to_string(),
        };
    }
}
impl From<StaffTypeData> for StaffType {
    fn from(value: StaffTypeData) -> Self {
        return Self {
            id: value.id.to_string(),
            staff_type: value.staff_type.to_string(),
        };
    }
}
impl From<&StaffType> for StaffTypeData {
    fn from(value: &StaffType) -> Self {
        let id = Uuid::parse_str(value.id.as_str());
        let staff_type_id = id.unwrap_or_else(|_| uuid::Uuid::nil());

        return Self {
            id: staff_type_id,
            staff_type: value.staff_type.to_string(),
        };
    }
}
impl From<StaffType> for StaffTypeData {
    fn from(value: StaffType) -> Self {
        let id = Uuid::parse_str(value.id.as_str());
        let staff_type_id = id.unwrap_or_else(|_| uuid::Uuid::nil());

        return Self {
            id: staff_type_id,
            staff_type: value.staff_type.to_string(),
        };
    }
}
impl From<&ContactTypeData> for ContactType {
    fn from(value: &ContactTypeData) -> Self {
        return Self {
            id: value.id.to_string(),
            contact_type: value.contact_type.to_string(),
        };
    }
}
impl From<ContactTypeData> for ContactType {
    fn from(value: ContactTypeData) -> Self {
        return Self {
            id: value.id.to_string(),
            contact_type: value.contact_type.to_string(),
        };
    }
}

impl From<&ContactType> for ContactTypeData {
    fn from(value: &ContactType) -> Self {
        let id = Uuid::parse_str(value.id.as_str());
        let contact_type_id = id.unwrap_or_else(|_| uuid::Uuid::nil());

        return Self {
            id: contact_type_id,
            contact_type: value.contact_type.to_string(),
        };
    }
}

impl From<ContactType> for ContactTypeData {
    fn from(value:ContactType) -> Self {
        let id = Uuid::parse_str(value.id.as_str());
        let contact_type_id = id.unwrap_or_else(|_| uuid::Uuid::nil());

        return Self {
            id: contact_type_id,
            contact_type: value.contact_type.to_string(),
        };
    }
}