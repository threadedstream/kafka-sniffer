mod kafka_sniffer_pcap;
mod kafka_sniffer_pcap_sys;

fn main() {
    // run kafka_sniffer_pcap::runner() or kafka_sniffer_smoltcp::runner()
    kafka_sniffer_pcap_sys::runner()
}
