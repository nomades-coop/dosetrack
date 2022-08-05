use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
// use mongodb::bson::RawArrayBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserRole {
    Admin,
    Operator,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CompanyStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperatorStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DosimeterStatus {
    Enabled,
    Disabled,
    InUse,
    InAnalisis,
    InCalibration,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DosimeterType {
    MOSFET,
    TLD,
    FILM,
    QUARTZ,
    GEIGER,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registration {
    pub company_name: String,
    pub company_cuit: String,
    pub user_name: String,
    pub user_last_name: String,
    pub user_email: Option<String>,
}

impl Registration {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub company_id: ObjectId,
    pub name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub role: UserRole,
    pub status: UserStatus,
}

impl User {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

// impl User {
//     pub fn new() -> Self {
//         Self {
//             _id: None,
//             company_id: ObjectId::new(),
//             name: String::new(),
//             last_name: String::new(),
//             role: UserRole::Operator,
//             status: UserStatus::Disabled,
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Operator {
    pub _id: Option<ObjectId>,
    pub company_id: ObjectId,
    pub name: String,
    pub dni: String,
    pub licence: String,
    pub dosimeter_id: Option<ObjectId>,
    pub film_id: Option<ObjectId>,
    pub status: OperatorStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Company {
    pub _id: Option<ObjectId>,
    pub cuit: String,
    pub name: String,
    pub operators: Option<Vec<Operator>>,
    pub status: CompanyStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Film {
    pub _id: Option<ObjectId>,
    pub company_id: ObjectId,
    pub operator_id: Option<ObjectId>,
    pub company_code: String,
    pub period: String,
    pub status: DosimeterStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dosimeter {
    pub _id: Option<ObjectId>,
    pub company_id: ObjectId,
    pub brand: String,
    pub model: String,
    pub sn: String,
    pub last_calibration_date: String,
    pub status: DosimeterStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dose {
    pub _id: Option<ObjectId>,
    pub company_id: ObjectId,
    pub operator_id: ObjectId,
    pub dosimeter_id: ObjectId,
    pub dose: f32,
    pub picture: Option<String>,
    pub when: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperationType {
    AddUser,
    RemoveUser,
    EditUser,
    AddDosimeter,
    RemoveDosimeter,
    EditDosimeter,
    AddFilm,
    RemoveFilm,
    EditFilm,
    AddDose,
    RemoveDose,
    EditDose,
    AddCompany,
    RemoveCompany,
    EditCompany,
    AddOperator,
    RemoveOperator,
    EditOperator,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auditrail {
    pub _id: Option<ObjectId>,
    pub company_id: Option<ObjectId>,
    pub operator_id: Option<ObjectId>,
    pub dosimeter_id: Option<ObjectId>,
    pub film_id: Option<ObjectId>,
    pub dose: Option<ObjectId>,
    pub operation: OperationType,
    pub when: DateTime,
}
