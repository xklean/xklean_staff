use crate::adapters::transfer::{AddressData, ContactData, ContactTypeData, StaffData, StaffTypeData};

impl From<StaffData> for pb_staff::Staff {
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

impl From<&StaffTypeData> for pb_staff::StaffType {
    fn from(value: &StaffTypeData) -> Self {
        return Self {
            id:value.id.to_string(),
            staff_type: value.staff_type.to_string(),
        };
    }
}

impl From<StaffTypeData> for pb_staff::StaffType {
    fn from(value: StaffTypeData) -> Self {
        return Self {
            id:value.id.to_string(),
            staff_type: value.staff_type.to_string(),
        };
    }
}

impl From<&ContactTypeData> for pb_staff::ContactType {
    fn from(value: &ContactTypeData) -> Self {
        return Self {
            id:value.id.to_string(),
            contact_type: value.contact_type.to_string(),
        };
    }
}

impl From<ContactTypeData> for pb_staff::ContactType {
    fn from(value: ContactTypeData) -> Self {
        return Self {
            id:value.id.to_string(),
            contact_type: value.contact_type.to_string(),
        };
    }
}