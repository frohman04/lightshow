trait CmdHandler {
    fn handle_command(&self, serial_device: &mut Box<dyn serialport::SerialPort>);
}
