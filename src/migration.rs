use serde::{Deserialize, Serialize};

use crate::imr::Field;

/**
The presentation of a migration file
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MigrationFile {
    /// The migration of the migration file
    pub migration: Migration,
}

/**
Representation for a migration.
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Migration {
    /// Hash of the migration
    pub hash: String,

    /// Marks the migration initial state
    pub initial: bool,

    /// ID of the migration, derived from filename
    #[serde(skip)]
    pub id: u16,

    /// Name of the migration, derived from filename
    #[serde(skip)]
    pub name: String,

    /// Migration this migration depends on
    pub dependency: Option<u16>,

    /// List of migrations this migration replaces
    pub replaces: Vec<u16>,

    /// The operations to execute
    pub operations: Vec<Operation>,
}

/**
The representation for all possible database operations
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum Operation {
    /// Representation of a CreateModel operation
    #[serde(rename_all = "PascalCase")]
    CreateModel {
        /// Name of the model
        name: String,
        /// List of fields associated to the model
        fields: Vec<Field>,
    },

    /// Representation of a RenameModel operation
    #[serde(rename_all = "PascalCase")]
    RenameModel {
        /// Old name of the model
        old: String,
        /// New name of the model
        new: String,
    },

    /// Representation of a DeleteModel operation
    #[serde(rename_all = "PascalCase")]
    DeleteModel {
        /// Name of the model
        name: String,
    },

    /// Representation of a CreateField operation
    #[serde(rename_all = "PascalCase")]
    CreateField {
        /// Name of the model
        model: String,
        /// The field that should be created
        field: Field,
    },

    /// Representation of a RenameField operation
    #[serde(rename_all = "PascalCase")]
    RenameField {
        /// Name of the table the column lives in
        table_name: String,

        /// Old name of the column
        old: String,

        /// New name of the column
        new: String,
    },

    /// Representation of a DeleteField operation
    #[serde(rename_all = "PascalCase")]
    DeleteField {
        /// Name of the model
        model: String,
        /// Name of the field to delete
        name: String,
    },

    /// Representation of a RawSQL operation
    #[serde(rename_all = "PascalCase")]
    RawSQL {
        /// The provided raw sql does not change the structure of the database.
        /// The migrator can assume that the layout stayed the same and will continue
        /// generating new migrations based on `.models.json`
        #[serde(default)]
        structure_safe: bool,
        /// SQL for sqlite
        #[serde(rename = "SQLite")]
        sqlite: String,
        /// SQL for postgres
        #[serde(rename = "Postgres")]
        postgres: String,
        /// SQL for mysql
        #[serde(rename = "MySQL")]
        mysql: String,
    },
}
