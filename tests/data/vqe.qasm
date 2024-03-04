// 89 lines 58 code 21 comments 10 blanks
/*
 * Variational eigensolver example
 *
 * Goal is to estimate the energy for a fixed set of parameters.
 * The parameters are updated outside of this program and a new
 * OpenQASM circuit is generated for the next iteration.
 */
include "stdgates.inc";

const int[32] n = 10;         // number of qubits
const int[32] layers = 3;     // number of entangler layers
const int[32] prec = 16;      // precision of all types
const int[32] shots = 1000;   // number of shots per Pauli observable

// Parameters could be written to local variables for this
// iteration, but we will request them using extern functions
extern get_parameter(uint[prec], uint[prec]) -> angle[prec];
extern get_npaulis() -> uint[prec];
extern get_pauli(int[prec]) -> bit[2 * n];

// The energy calculation uses floating point division,
// so we do that calculation in an extern function
extern update_energy(int[prec], uint[prec], float[prec]) -> float[prec];

gate entangler q { for uint i in [0:n-2] { cx q[i], q[i+1]; } }
def xmeasure(qubit q) -> bit { h q; return measure q; }
def ymeasure(qubit q) -> bit { s q; h q; return measure q; }

/* Pauli measurement circuit.
 * The first n-bits of spec are the X component.
 * The second n-bits of spec are the Z component.
 */
def pauli_measurement(bit[2*n] spec, qubit[n] q) -> bit {
  bit b = 0;
  for uint[prec] i in [0: n - 1] {
    bit temp;
    if(spec[i]==1 && spec[n+i]==0) { temp = xmeasure(q[i]); }
    if(spec[i]==0 && spec[n+i]==1) { temp = measure q[i]; }
    if(spec[i]==1 && spec[n+i]==1) { temp = ymeasure(q[i]); }
    b ^= temp;
  }
  return b;
}

// Circuit to prepare trial wave function
def trial_circuit(qubit[n] q) {
  for int[prec] l in [0: layers - 1] {
    for uint[prec] i in [0: n - 1] {
      angle[prec] theta;
      theta = get_parameter(l * layers + i);
      ry(theta) q[i];
    }
    if(l != layers - 1) entangler q;
  }
}

/* Apply VQE ansatz circuit and measure a Pauli operator
 * given by spec. Return the number of 1 outcomes.
 */
def counts_for_term(bit[2*n] spec, qubit[n] q) -> uint[prec] {
  uint[prec] counts;
  for uint i in [1: shots] {
    bit b;
    reset q;
    trial_circuit q;
    b = pauli_measurement(spec, q);
    counts += int[1](b);
  }
  return counts;
}

// Estimate the expected energy
def estimate_energy(qubit[n] q) -> float[prec] {
  float[prec] energy;
  uint[prec] npaulis = get_npaulis();
  for int[prec] t in [0:npaulis-1] {
    bit[2*n] spec = get_pauli(t);
    uint[prec] counts;
    counts = counts_for_term(spec, q);
    energy = update_energy(t, counts, energy);
  }
  return energy;
}

qubit[n] q;
float[prec] energy;

energy = estimate_energy(q);
