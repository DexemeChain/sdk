import { useState } from 'react'

export default function DepositForm() {
  const [amount, setAmount] = useState("")

  const handleDeposit = async () => {
    alert(`Deposit ${amount} SOL to Dexeme Index`)
    // TODO: integrate smart contract call
  }

  return (
    <div style={{ marginTop: 20 }}>
      <input
        type="number"
        placeholder="Amount in SOL"
        value={amount}
        onChange={(e) => setAmount(e.target.value)}
      />
      <button onClick={handleDeposit}>Deposit</button>
    </div>
  )
}
