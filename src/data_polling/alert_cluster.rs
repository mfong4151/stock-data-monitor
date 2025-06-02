/*
* A collection of alerts which are used transmit messages across functions
* Informs if an email alert should be fired or not,
* And what the contents of the email should be
*
* @is_volume_spike: shows if theres a volume spike or not
*
*/
pub struct AlertCluster{

 pub is_volume_spike: bool  
  
}

impl AlertCluster {

  pub fn is_alert_fireable(&self) -> bool {
    return self.is_volume_spike;
  }

}