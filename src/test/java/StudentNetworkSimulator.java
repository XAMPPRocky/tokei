
public class StudentNetworkSimulator extends NetworkSimulator {
    /*
     * Predefined Constants (static member variables):
     *
     *   int MAXDATASIZE : the maximum size of the Message data and
     *                     Packet payload
     *
     *   int A           : a predefined integer that represents entity A
     *   int B           : a predefined integer that represents entity B
     *
     *
     * Predefined Member Methods:
     *
     *  void stopTimer(int entity): 
     *       Stops the timer running at "entity" [A or B]
     *  void startTimer(int entity, double increment): 
     *       Starts a timer running at "entity" [A or B], which will expire in
     *       "increment" time units, causing the interrupt handler to be
     *       called.  You should only call this with A.
     *  void toLayer3(int callingEntity, Packet p)
     *       Puts the packet "p" into the network from "callingEntity" [A or B]
     *  void toLayer5(int entity, String dataSent)
     *       Passes "dataSent" up to layer 5 from "entity" [A or B]
     *  double getTime()
     *       Returns the current time in the simulator.  Might be useful for
     *       debugging.
     *  void printEventList()
     *       Prints the current event list to stdout.  Might be useful for
     *       debugging, but probably not.
     *
     *
     *  Predefined Classes:
     *
     *  Message: Used to encapsulate a message coming from layer 5
     *    Constructor:
     *      Message(String inputData): 
     *          creates a new Message containing "inputData"
     *    Methods:
     *      boolean setData(String inputData):
     *          sets an existing Message's data to "inputData"
     *          returns true on success, false otherwise
     *      String getData():
     *          returns the data contained in the message
     *  Packet: Used to encapsulate a packet
     *    Constructors:
     *      Packet (Packet p):
     *          creates a new Packet that is a copy of "p"
     *      Packet (int seq, int ack, int check, String newPayload)
     *          creates a new Packet with a sequence field of "seq", an
     *          ack field of "ack", a checksum field of "check", and a
     *          payload of "newPayload"
     *      Packet (int seq, int ack, int check)
     *          chreate a new Packet with a sequence field of "seq", an
     *          ack field of "ack", a checksum field of "check", and
     *          an empty payload
     *    Methods:
     *      boolean setSeqnum(int n)
     *          sets the Packet's sequence field to "n"
     *          returns true on success, false otherwise
     *      boolean setAcknum(int n)
     *          sets the Packet's ack field to "n"
     *          returns true on success, false otherwise
     *      boolean setChecksum(int n)
     *          sets the Packet's checksum to "n"
     *          returns true on success, false otherwise
     *      boolean setPayload(String newPayload)
     *          sets the Packet's payload to "newPayload"
     *          returns true on success, false otherwise
     *      int getSeqnum()
     *          returns the contents of the Packet's sequence field
     *      int getAcknum()
     *          returns the contents of the Packet's ack field
     *      int getChecksum()
     *          returns the checksum of the Packet
     *      String getPayload()
     *          returns the Packet's payload
     *
     */

    // Add any necessary class variables here.  Remember, you cannot use
    // these variables to send messages error free!  They can only hold
    // state information for A or B.
    // Also add any necessary methods (e.g. checksum of a String)
    
    // 'A' VARIABLES
    int aSeqNum;
    int aAckNum;
    boolean packetSent;
    boolean newPacket;
    Packet packet;

    private int createChecksum(Packet packet) {
        return packet.getSeqnum()
                + packet.getAcknum()
                + packet.getPayload().hashCode();
    }

    // This is the constructor.  Don't touch!
    public StudentNetworkSimulator(int numMessages,
            double loss,
            double corrupt,
            double avgDelay,
            int trace,
            long seed) {
        super(numMessages, loss, corrupt, avgDelay, trace, seed);
    }

    // This routine will be called whenever the upper layer at the sender [A]
    // has a message to send.  It is the job of your protocol to insure that
    // the data in such a message is delivered in-order, and correctly, to
    // the receiving upper layer. Return 1 if accepting the message to send, 
    // return 0 if refusing to send the message
    @Override
    protected int aOutput(Message message) {
        // Only create a packet if no packet is in transit,
        // and the last packet has been sent successfully.
        if (!packetSent && newPacket) {
            // Packet creation
            packet = new Packet(aSeqNum, aAckNum, 0, message.getData());
            packet.setChecksum(createChecksum(packet));
            debug("SENDING PACKETS");
            toLayer3(A, packet);
            startTimer(A, 1000);
            debug("NEW PACKET");
            packetSent = true;
            newPacket = false;
            return packetSent && !newPacket ? 1 : 0;
        } else {
            toLayer3(A, packet);
            return 0;
        }
    }

    // This routine will be called whenever a packet sent from the B-side 
    // (i.e. as a result of a toLayer3() being done by a B-side procedure)
    // arrives at the A-side.  "packet" is the (possibly corrupted) packet
    // sent from the B-side.
    @Override
    protected void aInput(Packet packet) {
        //***GETTING STARTED***
        // This will be needed later, when dealing with acknowledgments sent from B 
        if (packet.getChecksum() == createChecksum(packet)) {

            if (packet.getSeqnum() == aSeqNum) {
                stopTimer(A);
                aSeqNum = packet.getAcknum();
                aAckNum = packet.getSeqnum() + 1;
                packetSent = false;
                newPacket = true;
            } else {
                packetSent = false;
            }
        } else {
            packetSent = false;
        }
    }

    // This routine will be called when A's timer expires (thus generating a 
    // timer interrupt). You'll probably want to use this routine to control 
    // the retransmission of packets. See startTimer() and stopTimer(), above,
    // for how the timer is started and stopped. 
    @Override
    protected void aTimerInterrupt() {
        //***GETTING STARTED***
        // This will be needed later, to deal with lost packets
        debug("PACKET DROPPED SENDING ANOTHER");
        toLayer3(A, packet);
        packetSent = true;
        startTimer(A, 1000);
    }

    // This routine will be called once, before any of your other A-side 
    // routines are called. It can be used to do any required
    // initialization (e.g. of member variables you add to control the state
    // of entity A).
    @Override
    protected void aInit() {
        //***GETTING STARTED***
        // This will be needed later
        aSeqNum = 0;
        aAckNum = 0;
        newPacket = true;
    }
    
    // 'B' variables
    Packet lastPacket;
    String debugString;
    // This routine will be called whenever a packet sent from the A-side 
    // (i.e. as a result of a toLayer3() being done by an A-side procedure)
    // arrives at the B-side.  "packet" is the (possibly corrupted) packet
    // sent from the A-side.
    @Override
    protected void bInput(Packet packet) {
        //***GETTING STARTED***
        // To get started, extract the payload from the packet
        // and then send it up toLayer5
        
        // To see reliability search for LETTERS line in the output..
        // It takes the first letter of each packet and adds them to a string.
        // Example 26 Packets: abcdefghijklmnopqrstuvwxyz || abcdefghijklmnopqrstuvwxy
        // As the program begins to shutdown once the last packet has been received
        // And if the packet is corrupted or
        // dropped the program will end before it can be resent.
        if (packet.getChecksum() == createChecksum(packet)) {

            Packet ack = new Packet(packet.getAcknum(), packet.getSeqnum() + 1, 0, "ack");
            ack.setChecksum(createChecksum(ack));
            debug("SENDING ACKS");
            toLayer3(B, ack);
            if (lastPacket == null) {

                toLayer5(B, packet.getPayload());
                lastPacket = packet;
                debug("LETTERS: " + (debugString += packet.getPayload().substring(0, 1)));
            } else if (!packet.getPayload().equals(lastPacket.getPayload())) {
                lastPacket = packet;
                toLayer5(B, packet.getPayload());
                debug("LETTERS: " + (debugString += packet.getPayload().substring(0, 1)));
            }
        } else {

            Packet nack = new Packet(packet.getAcknum() - 1, packet.getSeqnum(), 0, "nack");
            nack.setChecksum(createChecksum(nack));
            toLayer3(B, nack);
            debug("SENDING NACKS");
        }
    }

    // This routine will be called once, before any of your other B-side 
    // routines are called. It can be used to do any required
    // initialization (e.g. of member variables you add to control the state
    // of entity B).
    @Override
    protected void bInit() {
        //***GETTING STARTED***
        // This will be needed later
        lastPacket = null;
        debugString = "";
    }

    private void debug(String message) {

        System.out.println("************************************");
        System.out.println(message);
        System.out.println("************************************");
    }
}
