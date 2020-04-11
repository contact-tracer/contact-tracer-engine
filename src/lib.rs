
extern crate chrono;

#[macro_use] extern crate serde_derive;


use chrono::{ NaiveDate };


// We create a type alias so that we always associate the same types to it
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        unsafe {
            let mut c_buf = 25;
            println!("{}",rust_wrapper_breadwallet_core::BRBIP38KeyIsValid(&mut c_buf));
            assert_eq!(0,rust_wrapper_breadwallet_core::BRBIP38KeyIsValid(&mut c_buf));
        }
    }
}


struct DiseaseParameters
{
	label: String,
	infectious_days_before_symptoms: u8,
	proximity_metres: u8,
	duration_mins: u8,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
struct ContactInstance
{
	our_contact_id: String,
	their_contact_id: String,
	timestamp: u64,
}


    /**
    Initialises the library for the specific disease parameters
    */
fn main(){

    let covid_19 = DiseaseParameters{
        label: String::from("COVID-19"),
        infectious_days_before_symptoms: 4,
        proximity_metres: 2,
        duration_mins: 15,
    };
}



/**
registers the method in your application that the library will call when it detects a notification, generally a change of infection status of a previous contact

# Arguments

method must of the type ... 

*/
pub fn register_notification_callback(callback: fn() ){
    todo!();
}

/**
Use this method to get the next available contactId. 

Your app gets a contact id from the library, ready to exchange it with the other party and receiving their contact id.

**Privacy Note:** The contact id is only ever used (ie exchanged) once. You must not persist contact pairs within your app, or pass those pairs to any server.

*/
pub fn get_contact_id(){
    todo!();
}


/**
Use this method to get a list next available contactIds. Use this method if ContactId generation is too slow, for example if the app is used in a high population area, or running on slow hardware.

Your app gets a contact id from the library, ready to negotiate exchanging it with the other party and receiving their contact id.

**Privacy Note:** Only use volatile memory for this list. You must not persist contact pairs within your app, or pass those pairs to any server.

*/
pub fn get_contact_id_list(size: u8){
    todo!();
}

/**

Your app tells the library when a contact has been registered. It does so by giving the pair of ids, in the order of your id, your contact's id, and the timestamp that the contact occurred.

Your contact tracing app uses your own definition of what a 'contact' is, for example proximity and duration. You also use your method of choice to identify contact, for example GPS, BLE Bluetooth Low Energy, QR code, manual check-in, etc.

NB have set timestamp to use u64 as we are expecting time since epoch
*/
pub fn record_contact(contact_pair: (&'static str,&'static str), timestamp: u64){
    todo!();

}


/**

set the infection status of your app user.

# Arguments

status [just traffic lights for now, pending WHO or similar saying how to structure this info]
	blue - no symptoms
	green - confirmed infection and recovered
	amber - suspected infection or has symptoms
	red - confirmed infection
symtomStartDate
testDate
testResult
testAttestation
testAttestationType
	self
	URL (eg hospital, testing service, or competent authority's server with hash.txt or similar)
	GPG or SLL (eg hospital's GPG or SSL sig of test result)
	etc
*/


pub fn set_infection_status(status: u8 ,test_date: NaiveDate, test_result: bool, test_signature: &str, test_signature_type: u8 ){
    todo!();
}

/**

set the tracing status of your app user. For example 

blue - unknown risk
green - low risk, eg negative test results, or over 14 days isolation
amber - high risk, eg has be in *close* contact with a potentially infected person (amber infection status), or any contact with a confirmed infected person (red infection status)
red - very high risk, eg has been in contact with a potentially infected person (amber infection status), or any contact with a confirmed infected person (red infection status)

*/
pub fn set_contact_trace_status(status: u8){
    todo!();
}

/**

get the tracing status of your app user.  If your callback has been notified of status changes of your contacts, then you should call this method.

blue - unknown risk
green - low risk, eg negative test results, or over 14 days isolation
amber - high risk, eg has be in *close* contact with a potentially infected person (amber infection status), or any contact with a confirmed infected person (red infection status)
red - very high risk, eg has been in contact with a potentially infected person (amber infection status), or any contact with a confirmed infected person (red infection status)

*/
pub fn get_contact_trace_status(){
    todo!();
}

/** 

After your app user has correctly set contact trace status and/or infection status, use this method to broadcast the changes to other contacts. 

Depending on local rules, your app may seek confirmation from a competent health provider or health authority before broadcasting, or may simply request user confirmation.

*Note:* Once broadcast, the broadcast cannot be revoked. However it can be superseded with subsequent test results.

*/
pub fn broadcast_contact_trace_status(){
    todo!();
}


/**

*/
pub fn get_funding_address(){
    todo!();
}
pub fn get_available_funds(){
    todo!();
}
pub fn withdraw_available_funds(address: &str){
    todo!();
}