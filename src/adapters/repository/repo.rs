use async_trait::async_trait;
use crate::adapters::errors;

#[async_trait]
pub trait IRepository {
   /// Retrieves a staff member by their unique identifier.
   ///
   /// # Arguments
   ///
   /// * `id` - The unique identifier of the staff member in the form of a `uuid::Uuid` type.
   ///
   /// # Examples
   ///
   /// ```
   /// use uuid::Uuid;
   /// use crate::data::Staff;
   /// use crate::errors::ServiceErr;
   ///
   /// # #[tokio::main]
   /// # async fn main() {
   /// #     let service = initialize_service().await;
   /// #
   /// #     match service.get_staff_by_id(Uuid::new_v4()).await {
   /// #         Ok(staff) => println!("Staff member found: {:?}", staff),
   /// #         Err(err) => println!("Error: {:?}", err),
   /// #     }
   /// # }
   /// ```
   ///
   /// # Errors
   ///
   /// Returns an error of type `errors::ServiceErr` if the staff member with the given id
   /// does not exist or if there is an issue retrieving the staff member.
   ///
   /// # Returns
   ///
   /// Returns a `Result` indicating either success with a `data::Staff` object or failure with an `errors::ServiceErr` object.
   async fn get_staff_by_id(&self,id:uuid::Uuid)->Result<data::Staff, errors::ServiceErr>;
   /// Retrieves contacts based on staff ID.
   ///
   /// # Arguments
   ///
   /// * `staff_id` - The ID of the staff member.
   ///
   /// # Returns
   ///
   /// Returns a `Result` containing a vector of `Contact` objects if successful, or a `ServiceErr` if an error occurs.
   ///
   /// # Examples
   ///
   /// ```rust
   /// use uuid::Uuid;
   /// use crate::data::Contact;
   /// use crate::errors::ServiceErr;
   ///
   /// #[tokio::main]
   /// async fn main() {
   ///     let service = Service::new();
   ///     let staff_id = Uuid::parse_str("62f44cd0-7b2b-4f55-beb2-ea3d61d5b274").unwrap();
   ///
   ///     let result = service.get_contacts_staff_id(staff_id).await;
   ///
   ///     match result {
   ///         Ok(contacts) => {
   ///             for contact in contacts {
   ///                 println!("{:?}", contact);
   ///             }
   ///         },
   ///         Err(err) => {
   ///             eprintln!("Error: {:?}", err);
   ///         }
   ///     }
   /// }
   /// ```
   async fn get_contacts_staff_id(&self,staff_id:uuid::Uuid)->Result<Vec<data::Contact>,errors::ServiceErr>;
   /// Asynchronously retrieves the address of a staff member identified by their `staff_id`.
   ///
   /// # Arguments
   ///
   /// * `staff_id` - The unique identifier of the staff member.
   ///
   /// # Returns
   ///
   /// A `Result` that contains a vector of `Staff` objects if successful,
   /// or a `ServiceErr` if an error occurs.
   ///
   /// # Examples
   ///
   /// ```rust,no_run
   /// # async fn example() -> Result<(), errors::ServiceErr> {
   /// let staff_id = uuid::Uuid::new_v4();
   /// let result = service.get_address_staff_id(staff_id).await?;
   /// # Ok(())
   /// # }
   /// ```
   async fn get_address_staff_id(&self, staff_id: uuid::Uuid) -> Result<Vec<data::Staff>, errors::ServiceErr>;
}