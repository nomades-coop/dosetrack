use mongodb::bson::oid::ObjectId;
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub company_id: ObjectId,
    pub name: String,
    pub last_name: String,
    pub role: UserRole,
    pub status: UserStatus,
}

impl User {
    pub fn new() -> Self {
        Self {
            _id: None,
            company_id: ObjectId::new(),
            name: String::new(),
            last_name: String::new(),
            role: UserRole::Operator,
            status: UserStatus::Disabled,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Operator {
    pub _id: ObjectId,
    pub company_id: ObjectId,
    pub name: String,
    pub dni: String,
    pub licence: String,
    pub dosimeter_id: ObjectId,
    pub status: OperatorStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Company {
    pub _id: ObjectId,
    pub name: String,
    pub users: Vec<User>,
    pub operators: Vec<Operator>,
    pub status: CompanyStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dosimeter {
    pub _id: ObjectId,
    pub brand: String,
    pub model: String,
    pub sn: f32,
    pub last_calibration_date: String,
    pub status: DosimeterStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dose {
    pub _id: ObjectId,
    pub operator_id: ObjectId,
    pub dosimeter_id: ObjectId,
    pub dose: f32,
    pub when: String,
    pub status: OperatorStatus,
}
