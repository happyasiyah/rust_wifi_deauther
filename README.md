# rust_wifi_deauther
A deauther ("jammer") implemented in Rust.

Using the fact that most access points are insecure in general, the ability to mimic 
client devices to transmit [Authentication Frames](https://mrncciew.com/2014/10/10/802-11-mgmt-authentication-frame/)
on behalf of other devices can act as a jammer in local areas. 

This will set up an available interface to monitor for devices connected to a subnet and send (1*) authentication frame to
the listening Access Point on behalf of each device and (1*) to each device on behalf of the Access Point.
