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

pub struct AlertClusterBuilder {
  is_volume_spike: bool  
}

impl AlertClusterBuilder {
    pub fn new() -> Self {
        AlertClusterBuilder {
            is_volume_spike: false, 
        }
    }

    pub fn set_is_volume_spike(mut self, is_volume_spike: bool) -> Self {
        self.is_volume_spike = is_volume_spike;
        self
    }

    //Creates a constraint on the volume spike
    // A condition that must be satisfied or it will not fire
    pub fn filter_volume_spike(mut self, is_alert_fireable: bool) -> Self{ 
      self.is_volume_spike = self.is_volume_spike && is_alert_fireable;
      self
    }

    pub fn build(self) -> AlertCluster {
        AlertCluster {
            is_volume_spike: self.is_volume_spike,
        }
    }
}


impl AlertCluster {
  pub fn is_alert_fireable(&self) -> bool {
    return self.is_volume_spike;
  }

}