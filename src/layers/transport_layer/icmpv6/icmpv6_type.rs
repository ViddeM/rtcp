use std::fmt::Display;

use colored::Colorize;

#[derive(Clone, Debug)]
pub enum ICMPv6Type {
    DestinationUnreachable,
    PacketTooBig,
    TimeExceeded,
    ParameterProblem,
    PrivateExperimentation,
    EchoRequest,
    EchoReply,
    MulticastListenerQuery,
    MulticastListenerReport,
    MulticastListenerDone,
    RouterSolicitation,
    RouterAdvertisement,
    NeighbourSolicitation,
    NeighbourAdvertisement,
    RedirectMessage,
    RouterRenumbering,
    ICMPNodeInformationQuery,
    ICMPNodeInformationResponse,
    InverseNeighborDiscoverySolicitationMessage,
    InverseNeighborDiscoveryAdvertisementMessage,
    Version2MulticastListenerReport,
    HomeAgentAddressDiscoveryRequestMessage,
    HomeAgentAddressDiscoveryReplyMessage,
    MobilePrefixSolicitation,
    MobilePrefixAdvertisement,
    CertificationPathSolicicationMessage,
    CertificationPathAdvertisementMessage,
    ExperimentalMobilityProtools,
    MulticastRouterAdvertisement,
    MulticastRouterSoliciation,
    MulticastRouterTermination,
    FMIPv6Messages,
    RPLControlMessage,
    ILNPv6LocatorUpdateMessage,
    DuplicateAddressRequest,
    DuplicateAddressConfirmation,
    MPLControlMessage,
    ExtendedEchoRequest,
    ExtendedEchoReply,
}

impl Display for ICMPv6Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ICMPv6Type::DestinationUnreachable => "Destination Unreachable",
                ICMPv6Type::PacketTooBig => "Packet too big",
                ICMPv6Type::TimeExceeded => "Time exceeded",
                ICMPv6Type::ParameterProblem => "Parameter problem",
                ICMPv6Type::PrivateExperimentation => "Private experimentation",
                ICMPv6Type::EchoRequest => "Echo request",
                ICMPv6Type::EchoReply => "Echo reply",
                ICMPv6Type::MulticastListenerQuery => "Multicast listener query",
                ICMPv6Type::MulticastListenerReport => "Multicast listener report",
                ICMPv6Type::MulticastListenerDone => "Multicast listener done",
                ICMPv6Type::RouterSolicitation => "Router solicitation",
                ICMPv6Type::RouterAdvertisement => "Router advertisement",
                ICMPv6Type::NeighbourSolicitation => "Neighbour solicitation",
                ICMPv6Type::NeighbourAdvertisement => "Neighbour advertisement",
                ICMPv6Type::RedirectMessage => "Redirect message",
                ICMPv6Type::RouterRenumbering => "Router renumbering",
                ICMPv6Type::ICMPNodeInformationQuery => "ICMP node information query",
                ICMPv6Type::ICMPNodeInformationResponse => "ICMP node information response",
                ICMPv6Type::InverseNeighborDiscoverySolicitationMessage =>
                    "Inverse neighbor discovery solicitation message",
                ICMPv6Type::InverseNeighborDiscoveryAdvertisementMessage =>
                    "Inverse neighbor discovery advertisement message",
                ICMPv6Type::Version2MulticastListenerReport =>
                    "Version 2 multicast listener report",
                ICMPv6Type::HomeAgentAddressDiscoveryRequestMessage =>
                    "Home agent address discovery request message",
                ICMPv6Type::HomeAgentAddressDiscoveryReplyMessage =>
                    "Home agent address discovery reply message",
                ICMPv6Type::MobilePrefixSolicitation => "Mobile prefix solicitation",
                ICMPv6Type::MobilePrefixAdvertisement => "Mobile prefix advertisement",
                ICMPv6Type::CertificationPathSolicicationMessage =>
                    "Certification path solicitation message",
                ICMPv6Type::CertificationPathAdvertisementMessage =>
                    "Certification path advertisement message",
                ICMPv6Type::ExperimentalMobilityProtools => "Experimental mobility protocols",
                ICMPv6Type::MulticastRouterAdvertisement => "Multicast router advertisement",
                ICMPv6Type::MulticastRouterSoliciation => "Multicast router solicitation",
                ICMPv6Type::MulticastRouterTermination => "Multicast router termination",
                ICMPv6Type::FMIPv6Messages => "FM IPv6 messages (Fast Mobile Handovers)",
                ICMPv6Type::RPLControlMessage =>
                    "RPL Control message (Routing Protocol for Low-Power and lossy networks)",
                ICMPv6Type::ILNPv6LocatorUpdateMessage =>
                    "ILNPv6 Locator Update Message (Identifier-Locator Network Protocol for IPv6)",
                ICMPv6Type::DuplicateAddressRequest => "Duplicate address request",
                ICMPv6Type::DuplicateAddressConfirmation => "Duplicate address confirmation",
                ICMPv6Type::MPLControlMessage =>
                    "MPL control message (Multicast Protocol for Low-Power and Lossy Networks)",
                ICMPv6Type::ExtendedEchoRequest => "Extended echo request",
                ICMPv6Type::ExtendedEchoReply => "Extended echo reply",
            }
        )
    }
}

impl ICMPv6Type {
    pub fn to_short_string(&self) -> String {
        format!("{}", self.to_string().yellow())
    }

    pub fn parse(message_type: u8, _code: u8, _message: &mut &[u8]) -> Option<Self> {
        Some(match message_type {
            1 => Self::DestinationUnreachable,
            2 => Self::PacketTooBig,
            3 => Self::TimeExceeded,
            4 => Self::ParameterProblem,
            100 | 101 | 200 | 201 => Self::PrivateExperimentation,
            0 | 127 | 255 => {
                eprintln!("Reserved type {message_type} used for ICMP v6");
                return None;
            }
            128 => Self::EchoRequest,
            129 => Self::EchoReply,
            130 => Self::MulticastListenerQuery,
            131 => Self::MulticastListenerReport,
            132 => Self::MulticastListenerDone,
            133 => Self::RouterSolicitation,
            134 => Self::RouterAdvertisement,
            135 => Self::NeighbourSolicitation,
            136 => Self::NeighbourAdvertisement,
            137 => Self::RedirectMessage,
            138 => Self::RouterRenumbering,
            139 => Self::ICMPNodeInformationQuery,
            140 => Self::ICMPNodeInformationResponse,
            141 => Self::InverseNeighborDiscoverySolicitationMessage,
            142 => Self::InverseNeighborDiscoveryAdvertisementMessage,
            143 => Self::Version2MulticastListenerReport,
            144 => Self::HomeAgentAddressDiscoveryRequestMessage,
            145 => Self::HomeAgentAddressDiscoveryReplyMessage,
            146 => Self::MobilePrefixSolicitation,
            147 => Self::MobilePrefixAdvertisement,
            148 => Self::CertificationPathSolicicationMessage,
            149 => Self::CertificationPathAdvertisementMessage,
            150 => Self::ExperimentalMobilityProtools,
            151 => Self::MulticastRouterAdvertisement,
            152 => Self::MulticastRouterSoliciation,
            153 => Self::MulticastRouterTermination,
            154 => Self::FMIPv6Messages,
            155 => Self::RPLControlMessage,
            156 => Self::ILNPv6LocatorUpdateMessage,
            157 => Self::DuplicateAddressRequest,
            158 => Self::DuplicateAddressConfirmation,
            159 => Self::MPLControlMessage,
            160 => Self::ExtendedEchoRequest,
            161 => Self::ExtendedEchoReply,
            n => {
                eprintln!("Invalid ICMP v6 type {n} used");
                return None;
            }
        })
    }
}
