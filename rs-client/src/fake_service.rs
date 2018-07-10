// Autogenerated by Thrift Compiler (0.11.0)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, type_complexity))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate ordered_float;
extern crate thrift;
extern crate try_from;

use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::From;
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use try_from::TryFrom;

use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

//
// InvalidOperation
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InvalidOperation {
  pub code: i32,
  pub why: String,
}

impl InvalidOperation {
  pub fn new(code: i32, why: String) -> InvalidOperation {
    InvalidOperation {
      code: code,
      why: why,
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<InvalidOperation> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<i32> = None;
    let mut f_2: Option<String> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i32()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("InvalidOperation.code", &f_1)?;
    verify_required_field_exists("InvalidOperation.why", &f_2)?;
    let ret = InvalidOperation {
      code: f_1.expect("auto-generated code should have checked for presence of required fields"),
      why: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("InvalidOperation");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("code", TType::I32, 1))?;
    o_prot.write_i32(self.code)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("why", TType::String, 2))?;
    o_prot.write_string(&self.why)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Error for InvalidOperation {
  fn description(&self) -> &str {
    "remote service threw InvalidOperation"
  }
}

impl From<InvalidOperation> for thrift::Error {
  fn from(e: InvalidOperation) -> Self {
    thrift::Error::User(Box::new(e))
  }
}

impl Display for InvalidOperation {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    self.description().fmt(f)
  }
}

//
// FakeProfile
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FakeProfile {
  pub id: i32,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub email: Option<String>,
}

impl FakeProfile {
  pub fn new<F2, F3, F4>(id: i32, first_name: F2, last_name: F3, email: F4) -> FakeProfile where F2: Into<Option<String>>, F3: Into<Option<String>>, F4: Into<Option<String>> {
    FakeProfile {
      id: id,
      first_name: first_name.into(),
      last_name: last_name.into(),
      email: email.into(),
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<FakeProfile> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<i32> = None;
    let mut f_2: Option<String> = None;
    let mut f_3: Option<String> = None;
    let mut f_4: Option<String> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i32()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        3 => {
          let val = i_prot.read_string()?;
          f_3 = Some(val);
        },
        4 => {
          let val = i_prot.read_string()?;
          f_4 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("FakeProfile.id", &f_1)?;
    let ret = FakeProfile {
      id: f_1.expect("auto-generated code should have checked for presence of required fields"),
      first_name: f_2,
      last_name: f_3,
      email: f_4,
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("FakeProfile");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("id", TType::I32, 1))?;
    o_prot.write_i32(self.id)?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.first_name {
      o_prot.write_field_begin(&TFieldIdentifier::new("first_name", TType::String, 2))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.last_name {
      o_prot.write_field_begin(&TFieldIdentifier::new("last_name", TType::String, 3))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.email {
      o_prot.write_field_begin(&TFieldIdentifier::new("email", TType::String, 4))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// FakeThingy service client
//

pub trait TFakeThingySyncClient {
  fn ping(&mut self) -> thrift::Result<()>;
  fn add(&mut self, num1: i32, num2: i32) -> thrift::Result<i32>;
  fn divide(&mut self, num1: OrderedFloat<f64>, num2: OrderedFloat<f64>) -> thrift::Result<OrderedFloat<f64>>;
  fn get_profile(&mut self, id: i32) -> thrift::Result<FakeProfile>;
}

pub trait TFakeThingySyncClientMarker {}

pub struct FakeThingySyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  _i_prot: IP,
  _o_prot: OP,
  _sequence_number: i32,
}

impl <IP, OP> FakeThingySyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  pub fn new(input_protocol: IP, output_protocol: OP) -> FakeThingySyncClient<IP, OP> {
    FakeThingySyncClient { _i_prot: input_protocol, _o_prot: output_protocol, _sequence_number: 0 }
  }
}

impl <IP, OP> TThriftClient for FakeThingySyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  fn i_prot_mut(&mut self) -> &mut TInputProtocol { &mut self._i_prot }
  fn o_prot_mut(&mut self) -> &mut TOutputProtocol { &mut self._o_prot }
  fn sequence_number(&self) -> i32 { self._sequence_number }
  fn increment_sequence_number(&mut self) -> i32 { self._sequence_number += 1; self._sequence_number }
}

impl <IP, OP> TFakeThingySyncClientMarker for FakeThingySyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}

impl <C: TThriftClient + TFakeThingySyncClientMarker> TFakeThingySyncClient for C {
  fn ping(&mut self) -> thrift::Result<()> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("ping", TMessageType::Call, self.sequence_number());
        let call_args = PingArgs {  };
        self.o_prot_mut().write_message_begin(&message_ident)?;
        call_args.write_to_out_protocol(self.o_prot_mut())?;
        self.o_prot_mut().write_message_end()?;
        self.o_prot_mut().flush()
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin()?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("ping", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut())?;
        self.i_prot_mut().read_message_end()?;
        return Err(thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = PingResult::read_from_in_protocol(self.i_prot_mut())?;
      self.i_prot_mut().read_message_end()?;
      result.ok_or()
    }
  }
  fn add(&mut self, num1: i32, num2: i32) -> thrift::Result<i32> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("add", TMessageType::Call, self.sequence_number());
        let call_args = AddArgs { num1: num1, num2: num2 };
        self.o_prot_mut().write_message_begin(&message_ident)?;
        call_args.write_to_out_protocol(self.o_prot_mut())?;
        self.o_prot_mut().write_message_end()?;
        self.o_prot_mut().flush()
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin()?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("add", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut())?;
        self.i_prot_mut().read_message_end()?;
        return Err(thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = AddResult::read_from_in_protocol(self.i_prot_mut())?;
      self.i_prot_mut().read_message_end()?;
      result.ok_or()
    }
  }
  fn divide(&mut self, num1: OrderedFloat<f64>, num2: OrderedFloat<f64>) -> thrift::Result<OrderedFloat<f64>> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("divide", TMessageType::Call, self.sequence_number());
        let call_args = DivideArgs { num1: num1, num2: num2 };
        self.o_prot_mut().write_message_begin(&message_ident)?;
        call_args.write_to_out_protocol(self.o_prot_mut())?;
        self.o_prot_mut().write_message_end()?;
        self.o_prot_mut().flush()
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin()?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("divide", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut())?;
        self.i_prot_mut().read_message_end()?;
        return Err(thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = DivideResult::read_from_in_protocol(self.i_prot_mut())?;
      self.i_prot_mut().read_message_end()?;
      result.ok_or()
    }
  }
  fn get_profile(&mut self, id: i32) -> thrift::Result<FakeProfile> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("get_profile", TMessageType::Call, self.sequence_number());
        let call_args = GetProfileArgs { id: id };
        self.o_prot_mut().write_message_begin(&message_ident)?;
        call_args.write_to_out_protocol(self.o_prot_mut())?;
        self.o_prot_mut().write_message_end()?;
        self.o_prot_mut().flush()
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin()?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("get_profile", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut())?;
        self.i_prot_mut().read_message_end()?;
        return Err(thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = GetProfileResult::read_from_in_protocol(self.i_prot_mut())?;
      self.i_prot_mut().read_message_end()?;
      result.ok_or()
    }
  }
}

//
// FakeThingy service processor
//

pub trait FakeThingySyncHandler {
  fn handle_ping(&self) -> thrift::Result<()>;
  fn handle_add(&self, num1: i32, num2: i32) -> thrift::Result<i32>;
  fn handle_divide(&self, num1: OrderedFloat<f64>, num2: OrderedFloat<f64>) -> thrift::Result<OrderedFloat<f64>>;
  fn handle_get_profile(&self, id: i32) -> thrift::Result<FakeProfile>;
}

pub struct FakeThingySyncProcessor<H: FakeThingySyncHandler> {
  handler: H,
}

impl <H: FakeThingySyncHandler> FakeThingySyncProcessor<H> {
  pub fn new(handler: H) -> FakeThingySyncProcessor<H> {
    FakeThingySyncProcessor {
      handler: handler,
    }
  }
  fn process_ping(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    TFakeThingyProcessFunctions::process_ping(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
  fn process_add(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    TFakeThingyProcessFunctions::process_add(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
  fn process_divide(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    TFakeThingyProcessFunctions::process_divide(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
  fn process_get_profile(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    TFakeThingyProcessFunctions::process_get_profile(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
}

pub struct TFakeThingyProcessFunctions;

impl TFakeThingyProcessFunctions {
  pub fn process_ping<H: FakeThingySyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let _ = PingArgs::read_from_in_protocol(i_prot)?;
    match handler.handle_ping() {
      Ok(_) => {
        let message_ident = TMessageIdentifier::new("ping", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident)?;
        let ret = PingResult {  };
        ret.write_to_out_protocol(o_prot)?;
        o_prot.write_message_end()?;
        o_prot.flush()
      },
      Err(e) => {
        match e {
          thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("ping", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.description()
              )
            };
            let message_ident = TMessageIdentifier::new("ping", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
        }
      },
    }
  }
  pub fn process_add<H: FakeThingySyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let args = AddArgs::read_from_in_protocol(i_prot)?;
    match handler.handle_add(args.num1, args.num2) {
      Ok(handler_return) => {
        let message_ident = TMessageIdentifier::new("add", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident)?;
        let ret = AddResult { result_value: Some(handler_return) };
        ret.write_to_out_protocol(o_prot)?;
        o_prot.write_message_end()?;
        o_prot.flush()
      },
      Err(e) => {
        match e {
          thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("add", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.description()
              )
            };
            let message_ident = TMessageIdentifier::new("add", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
        }
      },
    }
  }
  pub fn process_divide<H: FakeThingySyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let args = DivideArgs::read_from_in_protocol(i_prot)?;
    match handler.handle_divide(args.num1, args.num2) {
      Ok(handler_return) => {
        let message_ident = TMessageIdentifier::new("divide", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident)?;
        let ret = DivideResult { result_value: Some(handler_return), ouch: None };
        ret.write_to_out_protocol(o_prot)?;
        o_prot.write_message_end()?;
        o_prot.flush()
      },
      Err(e) => {
        match e {
          thrift::Error::User(usr_err) => {
            if usr_err.downcast_ref::<InvalidOperation>().is_some() {
              let err = usr_err.downcast::<InvalidOperation>().expect("downcast already checked");
              let ret_err = DivideResult{ result_value: None, ouch: Some(*err) };
              let message_ident = TMessageIdentifier::new("divide", TMessageType::Reply, incoming_sequence_number);
              o_prot.write_message_begin(&message_ident)?;
              ret_err.write_to_out_protocol(o_prot)?;
              o_prot.write_message_end()?;
              o_prot.flush()
            } else {
              let ret_err = {
                ApplicationError::new(
                  ApplicationErrorKind::Unknown,
                  usr_err.description()
                )
              };
              let message_ident = TMessageIdentifier::new("divide", TMessageType::Exception, incoming_sequence_number);
              o_prot.write_message_begin(&message_ident)?;
              thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
              o_prot.write_message_end()?;
              o_prot.flush()
            }
          },
          thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("divide", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.description()
              )
            };
            let message_ident = TMessageIdentifier::new("divide", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
        }
      },
    }
  }
  pub fn process_get_profile<H: FakeThingySyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let args = GetProfileArgs::read_from_in_protocol(i_prot)?;
    match handler.handle_get_profile(args.id) {
      Ok(handler_return) => {
        let message_ident = TMessageIdentifier::new("get_profile", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident)?;
        let ret = GetProfileResult { result_value: Some(handler_return) };
        ret.write_to_out_protocol(o_prot)?;
        o_prot.write_message_end()?;
        o_prot.flush()
      },
      Err(e) => {
        match e {
          thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("get_profile", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.description()
              )
            };
            let message_ident = TMessageIdentifier::new("get_profile", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
        }
      },
    }
  }
}

impl <H: FakeThingySyncHandler> TProcessor for FakeThingySyncProcessor<H> {
  fn process(&self, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let message_ident = i_prot.read_message_begin()?;
    let res = match &*message_ident.name {
      "ping" => {
        self.process_ping(message_ident.sequence_number, i_prot, o_prot)
      },
      "add" => {
        self.process_add(message_ident.sequence_number, i_prot, o_prot)
      },
      "divide" => {
        self.process_divide(message_ident.sequence_number, i_prot, o_prot)
      },
      "get_profile" => {
        self.process_get_profile(message_ident.sequence_number, i_prot, o_prot)
      },
      method => {
        Err(
          thrift::Error::Application(
            ApplicationError::new(
              ApplicationErrorKind::UnknownMethod,
              format!("unknown method {}", method)
            )
          )
        )
      },
    };
    thrift::server::handle_process_result(&message_ident, res, o_prot)
  }
}

//
// PingArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct PingArgs {
}

impl PingArgs {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<PingArgs> {
    i_prot.read_struct_begin()?;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = PingArgs {};
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("ping_args");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// PingResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct PingResult {
}

impl PingResult {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<PingResult> {
    i_prot.read_struct_begin()?;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = PingResult {};
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("PingResult");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
  fn ok_or(self) -> thrift::Result<()> {
    Ok(())
  }
}

//
// AddArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct AddArgs {
  num1: i32,
  num2: i32,
}

impl AddArgs {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<AddArgs> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<i32> = None;
    let mut f_2: Option<i32> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i32()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_i32()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("AddArgs.num1", &f_1)?;
    verify_required_field_exists("AddArgs.num2", &f_2)?;
    let ret = AddArgs {
      num1: f_1.expect("auto-generated code should have checked for presence of required fields"),
      num2: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("add_args");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("num1", TType::I32, 1))?;
    o_prot.write_i32(self.num1)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("num2", TType::I32, 2))?;
    o_prot.write_i32(self.num2)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// AddResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct AddResult {
  result_value: Option<i32>,
}

impl AddResult {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<AddResult> {
    i_prot.read_struct_begin()?;
    let mut f_0: Option<i32> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        0 => {
          let val = i_prot.read_i32()?;
          f_0 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = AddResult {
      result_value: f_0,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("AddResult");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(fld_var) = self.result_value {
      o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::I32, 0))?;
      o_prot.write_i32(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
  fn ok_or(self) -> thrift::Result<i32> {
    if self.result_value.is_some() {
      Ok(self.result_value.unwrap())
    } else {
      Err(
        thrift::Error::Application(
          ApplicationError::new(
            ApplicationErrorKind::MissingResult,
            "no result received for Add"
          )
        )
      )
    }
  }
}

//
// DivideArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct DivideArgs {
  num1: OrderedFloat<f64>,
  num2: OrderedFloat<f64>,
}

impl DivideArgs {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<DivideArgs> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<OrderedFloat<f64>> = None;
    let mut f_2: Option<OrderedFloat<f64>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = OrderedFloat::from(i_prot.read_double()?);
          f_1 = Some(val);
        },
        2 => {
          let val = OrderedFloat::from(i_prot.read_double()?);
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("DivideArgs.num1", &f_1)?;
    verify_required_field_exists("DivideArgs.num2", &f_2)?;
    let ret = DivideArgs {
      num1: f_1.expect("auto-generated code should have checked for presence of required fields"),
      num2: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("divide_args");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("num1", TType::Double, 1))?;
    o_prot.write_double(self.num1.into())?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("num2", TType::Double, 2))?;
    o_prot.write_double(self.num2.into())?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// DivideResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct DivideResult {
  result_value: Option<OrderedFloat<f64>>,
  ouch: Option<InvalidOperation>,
}

impl DivideResult {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<DivideResult> {
    i_prot.read_struct_begin()?;
    let mut f_0: Option<OrderedFloat<f64>> = None;
    let mut f_1: Option<InvalidOperation> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        0 => {
          let val = OrderedFloat::from(i_prot.read_double()?);
          f_0 = Some(val);
        },
        1 => {
          let val = InvalidOperation::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = DivideResult {
      result_value: f_0,
      ouch: f_1,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("DivideResult");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(fld_var) = self.result_value {
      o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::Double, 0))?;
      o_prot.write_double(fld_var.into())?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.ouch {
      o_prot.write_field_begin(&TFieldIdentifier::new("ouch", TType::Struct, 1))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
  fn ok_or(self) -> thrift::Result<OrderedFloat<f64>> {
    if self.ouch.is_some() {
      Err(thrift::Error::User(Box::new(self.ouch.unwrap())))
    } else if self.result_value.is_some() {
      Ok(self.result_value.unwrap())
    } else {
      Err(
        thrift::Error::Application(
          ApplicationError::new(
            ApplicationErrorKind::MissingResult,
            "no result received for Divide"
          )
        )
      )
    }
  }
}

//
// GetProfileArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct GetProfileArgs {
  id: i32,
}

impl GetProfileArgs {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<GetProfileArgs> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<i32> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i32()?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("GetProfileArgs.id", &f_1)?;
    let ret = GetProfileArgs {
      id: f_1.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("get_profile_args");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("id", TType::I32, 1))?;
    o_prot.write_i32(self.id)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// GetProfileResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct GetProfileResult {
  result_value: Option<FakeProfile>,
}

impl GetProfileResult {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<GetProfileResult> {
    i_prot.read_struct_begin()?;
    let mut f_0: Option<FakeProfile> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        0 => {
          let val = FakeProfile::read_from_in_protocol(i_prot)?;
          f_0 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = GetProfileResult {
      result_value: f_0,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("GetProfileResult");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.result_value {
      o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::Struct, 0))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
  fn ok_or(self) -> thrift::Result<FakeProfile> {
    if self.result_value.is_some() {
      Ok(self.result_value.unwrap())
    } else {
      Err(
        thrift::Error::Application(
          ApplicationError::new(
            ApplicationErrorKind::MissingResult,
            "no result received for GetProfile"
          )
        )
      )
    }
  }
}
