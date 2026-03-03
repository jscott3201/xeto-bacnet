#[cfg(test)]
mod tests {
    use haystack_core::ontology::DefNamespace;
    use std::fs;
    use std::path::Path;

    fn load_bacnet_lib(ns: &mut DefNamespace, filename: &str, lib_name: &str) {
        let xeto_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/bacnet");
        let source = fs::read_to_string(xeto_dir.join(filename))
            .unwrap_or_else(|e| panic!("Failed to read {filename}: {e}"));
        ns.load_xeto_str(&source, lib_name)
            .unwrap_or_else(|e| panic!("Failed to parse {filename}: {e}"));
    }

    #[test]
    fn test_lib_pragma_loads() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
    }

    #[test]
    fn test_enums_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "enums.xeto", "bacnet.enums");
        assert!(ns.get_spec("bacnet.enums::ObjectType").is_some());
        assert!(ns.get_spec("bacnet.enums::Reliability").is_some());
        assert!(ns.get_spec("bacnet.enums::EventState").is_some());
    }

    #[test]
    fn test_device_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "device.xeto", "bacnet.device");
        assert!(ns.get_spec("bacnet.device::BacnetDevice").is_some());
    }

    #[test]
    fn test_object_base_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/base.xeto", "bacnet.objects.base");
        assert!(ns.get_spec("bacnet.objects.base::BacnetObject").is_some());
    }

    #[test]
    fn test_transport_and_network_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "transport.xeto", "bacnet.transport");
        load_bacnet_lib(&mut ns, "network.xeto", "bacnet.network");
        assert!(ns.get_spec("bacnet.transport::BacnetPort").is_some());
        assert!(ns.get_spec("bacnet.transport::BIPPort").is_some());
        assert!(ns.get_spec("bacnet.transport::MSTPPort").is_some());
        assert!(ns.get_spec("bacnet.network::BacnetNetwork").is_some());
    }

    #[test]
    fn test_analog_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/analog.xeto", "bacnet.objects.analog");
        assert!(ns.get_spec("bacnet.objects.analog::AnalogInput").is_some());
        assert!(ns.get_spec("bacnet.objects.analog::AnalogOutput").is_some());
        assert!(ns.get_spec("bacnet.objects.analog::AnalogValue").is_some());
        assert!(
            ns.get_spec("bacnet.objects.analog::LargeAnalogValue")
                .is_some()
        );
    }

    #[test]
    fn test_binary_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/binary.xeto", "bacnet.objects.binary");
        assert!(ns.get_spec("bacnet.objects.binary::BinaryInput").is_some());
        assert!(ns.get_spec("bacnet.objects.binary::BinaryOutput").is_some());
        assert!(ns.get_spec("bacnet.objects.binary::BinaryValue").is_some());
        assert!(
            ns.get_spec("bacnet.objects.binary::BinaryLightingOutput")
                .is_some()
        );
    }

    #[test]
    fn test_multistate_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(
            &mut ns,
            "objects/multistate.xeto",
            "bacnet.objects.multistate",
        );
        assert!(
            ns.get_spec("bacnet.objects.multistate::MultiStateInput")
                .is_some()
        );
        assert!(
            ns.get_spec("bacnet.objects.multistate::MultiStateOutput")
                .is_some()
        );
        assert!(
            ns.get_spec("bacnet.objects.multistate::MultiStateValue")
                .is_some()
        );
    }

    #[test]
    fn test_schedule_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/schedule.xeto", "bacnet.objects.schedule");
        assert!(ns.get_spec("bacnet.objects.schedule::Schedule").is_some());
        assert!(ns.get_spec("bacnet.objects.schedule::Calendar").is_some());
    }

    #[test]
    fn test_trend_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/trend.xeto", "bacnet.objects.trend");
        assert!(ns.get_spec("bacnet.objects.trend::TrendLog").is_some());
        assert!(
            ns.get_spec("bacnet.objects.trend::TrendLogMultiple")
                .is_some()
        );
        assert!(ns.get_spec("bacnet.objects.trend::EventLog").is_some());
    }

    #[test]
    fn test_notification_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(
            &mut ns,
            "objects/notification.xeto",
            "bacnet.objects.notification",
        );
        assert!(
            ns.get_spec("bacnet.objects.notification::NotificationClass")
                .is_some()
        );
        assert!(
            ns.get_spec("bacnet.objects.notification::EventEnrollment")
                .is_some()
        );
        assert!(
            ns.get_spec("bacnet.objects.notification::AlertEnrollment")
                .is_some()
        );
        assert!(
            ns.get_spec("bacnet.objects.notification::NotificationForwarder")
                .is_some()
        );
    }

    #[test]
    fn test_loop_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/loop.xeto", "bacnet.objects.loop");
        assert!(ns.get_spec("bacnet.objects.loop::Loop").is_some());
        assert!(ns.get_spec("bacnet.objects.loop::Command").is_some());
    }

    #[test]
    fn test_file_object_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/file.xeto", "bacnet.objects.file");
        assert!(ns.get_spec("bacnet.objects.file::File").is_some());
    }

    #[test]
    fn test_program_object_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/program.xeto", "bacnet.objects.program");
        assert!(ns.get_spec("bacnet.objects.program::Program").is_some());
    }

    #[test]
    fn test_network_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/network.xeto", "bacnet.objects.network");
        assert!(ns.get_spec("bacnet.objects.network::NetworkPort").is_some());
        assert!(
            ns.get_spec("bacnet.objects.network::NetworkSecurity")
                .is_some()
        );
    }

    #[test]
    fn test_access_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/access.xeto", "bacnet.objects.access");
        assert!(ns.get_spec("bacnet.objects.access::AccessDoor").is_some());
        assert!(ns.get_spec("bacnet.objects.access::AccessPoint").is_some());
        assert!(ns.get_spec("bacnet.objects.access::AccessZone").is_some());
        assert!(
            ns.get_spec("bacnet.objects.access::AccessCredential")
                .is_some()
        );
        assert!(ns.get_spec("bacnet.objects.access::AccessRights").is_some());
        assert!(ns.get_spec("bacnet.objects.access::AccessUser").is_some());
        assert!(
            ns.get_spec("bacnet.objects.access::CredentialDataInput")
                .is_some()
        );
    }

    #[test]
    fn test_lighting_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/lighting.xeto", "bacnet.objects.lighting");
        assert!(
            ns.get_spec("bacnet.objects.lighting::LightingOutput")
                .is_some()
        );
        assert!(ns.get_spec("bacnet.objects.lighting::Channel").is_some());
    }

    #[test]
    fn test_elevator_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/elevator.xeto", "bacnet.objects.elevator");
        assert!(
            ns.get_spec("bacnet.objects.elevator::ElevatorGroup")
                .is_some()
        );
        assert!(ns.get_spec("bacnet.objects.elevator::Escalator").is_some());
        assert!(ns.get_spec("bacnet.objects.elevator::Lift").is_some());
        assert!(ns.get_spec("bacnet.objects.elevator::Staging").is_some());
    }

    #[test]
    fn test_value_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/value.xeto", "bacnet.objects.value");
        assert!(
            ns.get_spec("bacnet.objects.value::BitstringValue")
                .is_some()
        );
        assert!(
            ns.get_spec("bacnet.objects.value::CharacterstringValue")
                .is_some()
        );
        assert!(ns.get_spec("bacnet.objects.value::DateValue").is_some());
        assert!(ns.get_spec("bacnet.objects.value::DatetimeValue").is_some());
        assert!(
            ns.get_spec("bacnet.objects.value::DatePatternValue")
                .is_some()
        );
        assert!(
            ns.get_spec("bacnet.objects.value::DatetimePatternValue")
                .is_some()
        );
        assert!(ns.get_spec("bacnet.objects.value::IntegerValue").is_some());
        assert!(
            ns.get_spec("bacnet.objects.value::OctetstringValue")
                .is_some()
        );
        assert!(
            ns.get_spec("bacnet.objects.value::PositiveIntegerValue")
                .is_some()
        );
        assert!(ns.get_spec("bacnet.objects.value::TimeValue").is_some());
        assert!(
            ns.get_spec("bacnet.objects.value::TimePatternValue")
                .is_some()
        );
    }

    #[test]
    fn test_safety_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/safety.xeto", "bacnet.objects.safety");
        assert!(
            ns.get_spec("bacnet.objects.safety::LifeSafetyPoint")
                .is_some()
        );
        assert!(
            ns.get_spec("bacnet.objects.safety::LifeSafetyZone")
                .is_some()
        );
    }

    #[test]
    fn test_counter_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/counter.xeto", "bacnet.objects.counter");
        assert!(ns.get_spec("bacnet.objects.counter::Accumulator").is_some());
        assert!(
            ns.get_spec("bacnet.objects.counter::PulseConverter")
                .is_some()
        );
        assert!(ns.get_spec("bacnet.objects.counter::Averaging").is_some());
    }

    #[test]
    fn test_load_control_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/load.xeto", "bacnet.objects.load");
        assert!(ns.get_spec("bacnet.objects.load::LoadControl").is_some());
    }

    #[test]
    fn test_group_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/group.xeto", "bacnet.objects.group");
        assert!(ns.get_spec("bacnet.objects.group::Group").is_some());
        assert!(ns.get_spec("bacnet.objects.group::GlobalGroup").is_some());
        assert!(
            ns.get_spec("bacnet.objects.group::StructuredView")
                .is_some()
        );
    }

    #[test]
    fn test_timer_object_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/timer.xeto", "bacnet.objects.timer");
        assert!(ns.get_spec("bacnet.objects.timer::Timer").is_some());
    }

    #[test]
    fn test_audit_objects_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "objects/audit.xeto", "bacnet.objects.audit");
        assert!(ns.get_spec("bacnet.objects.audit::AuditLog").is_some());
        assert!(ns.get_spec("bacnet.objects.audit::AuditReporter").is_some());
    }

    #[test]
    fn test_services_load() {
        let mut ns = DefNamespace::load_standard().unwrap();
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");
        load_bacnet_lib(&mut ns, "services.xeto", "bacnet.services");
        assert!(ns.get_spec("bacnet.services::BacnetService").is_some());
        assert!(
            ns.get_spec("bacnet.services::ReadPropertyService")
                .is_some()
        );
        assert!(ns.get_spec("bacnet.services::WhoIsService").is_some());
        assert!(
            ns.get_spec("bacnet.services::SubscribeCovService")
                .is_some()
        );
        assert!(
            ns.get_spec("bacnet.services::AuditLogQueryService")
                .is_some()
        );
    }

    /// Full integration test: loads ALL 27 libs in dependency order and
    /// spot-checks types across every layer of the hierarchy.
    #[test]
    fn test_full_integration_all_libs() {
        let mut ns = DefNamespace::load_standard().unwrap();

        // Layer 0: root
        load_bacnet_lib(&mut ns, "lib.xeto", "bacnet");

        // Layer 1: enums, transport, network, device
        load_bacnet_lib(&mut ns, "enums.xeto", "bacnet.enums");
        load_bacnet_lib(&mut ns, "transport.xeto", "bacnet.transport");
        load_bacnet_lib(&mut ns, "network.xeto", "bacnet.network");
        load_bacnet_lib(&mut ns, "device.xeto", "bacnet.device");

        // Layer 2: object base + services
        load_bacnet_lib(&mut ns, "objects/base.xeto", "bacnet.objects.base");
        load_bacnet_lib(&mut ns, "services.xeto", "bacnet.services");

        // Layer 3: all object type files (21 libs)
        load_bacnet_lib(&mut ns, "objects/analog.xeto", "bacnet.objects.analog");
        load_bacnet_lib(&mut ns, "objects/binary.xeto", "bacnet.objects.binary");
        load_bacnet_lib(
            &mut ns,
            "objects/multistate.xeto",
            "bacnet.objects.multistate",
        );
        load_bacnet_lib(&mut ns, "objects/schedule.xeto", "bacnet.objects.schedule");
        load_bacnet_lib(&mut ns, "objects/trend.xeto", "bacnet.objects.trend");
        load_bacnet_lib(
            &mut ns,
            "objects/notification.xeto",
            "bacnet.objects.notification",
        );
        load_bacnet_lib(&mut ns, "objects/loop.xeto", "bacnet.objects.loop");
        load_bacnet_lib(&mut ns, "objects/file.xeto", "bacnet.objects.file");
        load_bacnet_lib(&mut ns, "objects/program.xeto", "bacnet.objects.program");
        load_bacnet_lib(&mut ns, "objects/network.xeto", "bacnet.objects.network");
        load_bacnet_lib(&mut ns, "objects/access.xeto", "bacnet.objects.access");
        load_bacnet_lib(&mut ns, "objects/lighting.xeto", "bacnet.objects.lighting");
        load_bacnet_lib(&mut ns, "objects/elevator.xeto", "bacnet.objects.elevator");
        load_bacnet_lib(&mut ns, "objects/value.xeto", "bacnet.objects.value");
        load_bacnet_lib(&mut ns, "objects/safety.xeto", "bacnet.objects.safety");
        load_bacnet_lib(&mut ns, "objects/counter.xeto", "bacnet.objects.counter");
        load_bacnet_lib(&mut ns, "objects/load.xeto", "bacnet.objects.load");
        load_bacnet_lib(&mut ns, "objects/group.xeto", "bacnet.objects.group");
        load_bacnet_lib(&mut ns, "objects/timer.xeto", "bacnet.objects.timer");
        load_bacnet_lib(&mut ns, "objects/audit.xeto", "bacnet.objects.audit");

        // Spot-check: one type from each layer
        assert!(
            ns.get_spec("bacnet.enums::ObjectType").is_some(),
            "enum ObjectType"
        );
        assert!(
            ns.get_spec("bacnet.transport::BIPPort").is_some(),
            "transport BIPPort"
        );
        assert!(
            ns.get_spec("bacnet.network::BacnetRouter").is_some(),
            "network BacnetRouter"
        );
        assert!(
            ns.get_spec("bacnet.device::BacnetDevice").is_some(),
            "device BacnetDevice"
        );
        assert!(
            ns.get_spec("bacnet.objects.base::BacnetObject").is_some(),
            "base BacnetObject"
        );
        assert!(
            ns.get_spec("bacnet.services::ReadPropertyService")
                .is_some(),
            "service ReadProperty"
        );

        // Spot-check: one type from each object file
        assert!(
            ns.get_spec("bacnet.objects.analog::AnalogInput").is_some(),
            "AnalogInput"
        );
        assert!(
            ns.get_spec("bacnet.objects.binary::BinaryOutput").is_some(),
            "BinaryOutput"
        );
        assert!(
            ns.get_spec("bacnet.objects.multistate::MultiStateValue")
                .is_some(),
            "MultiStateValue"
        );
        assert!(
            ns.get_spec("bacnet.objects.schedule::Schedule").is_some(),
            "Schedule"
        );
        assert!(
            ns.get_spec("bacnet.objects.trend::TrendLog").is_some(),
            "TrendLog"
        );
        assert!(
            ns.get_spec("bacnet.objects.notification::NotificationClass")
                .is_some(),
            "NotificationClass"
        );
        assert!(ns.get_spec("bacnet.objects.loop::Loop").is_some(), "Loop");
        assert!(ns.get_spec("bacnet.objects.file::File").is_some(), "File");
        assert!(
            ns.get_spec("bacnet.objects.program::Program").is_some(),
            "Program"
        );
        assert!(
            ns.get_spec("bacnet.objects.network::NetworkPort").is_some(),
            "NetworkPort"
        );
        assert!(
            ns.get_spec("bacnet.objects.access::AccessDoor").is_some(),
            "AccessDoor"
        );
        assert!(
            ns.get_spec("bacnet.objects.lighting::LightingOutput")
                .is_some(),
            "LightingOutput"
        );
        assert!(
            ns.get_spec("bacnet.objects.elevator::Lift").is_some(),
            "Lift"
        );
        assert!(
            ns.get_spec("bacnet.objects.value::IntegerValue").is_some(),
            "IntegerValue"
        );
        assert!(
            ns.get_spec("bacnet.objects.safety::LifeSafetyPoint")
                .is_some(),
            "LifeSafetyPoint"
        );
        assert!(
            ns.get_spec("bacnet.objects.counter::Accumulator").is_some(),
            "Accumulator"
        );
        assert!(
            ns.get_spec("bacnet.objects.load::LoadControl").is_some(),
            "LoadControl"
        );
        assert!(
            ns.get_spec("bacnet.objects.group::StructuredView")
                .is_some(),
            "StructuredView"
        );
        assert!(
            ns.get_spec("bacnet.objects.timer::Timer").is_some(),
            "Timer"
        );
        assert!(
            ns.get_spec("bacnet.objects.audit::AuditLog").is_some(),
            "AuditLog"
        );
    }
}
