use crate::db;
use crate::db::*; 
use actix_web::{web, App, http, HttpResponse, HttpServer, Responder, test};
use deadpool_postgres::{Config, Pool};
use serde::Deserialize;
use serde_json::json;
use tokio_postgres::{Error, NoTls, Row};
use super::MyDbError;
//////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////
////////////// *****  Session Route Handler Functions ***** //////////////////////
//////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////

/// Create_session
/// To start a new user session upon login

/// End_session
/// To allow users to log out OR just end the session after a certain amount of
/// time OR end session after a certain amount of inactivity.

/// Get_active_sessions
/// To retrieve active sessions, useful for administrative purposes.
