import WalletButton from '../components/WalletButton'
import DepositForm from '../components/DepositForm'

export default function Home() {
  return (
    <div style={{ padding: 40 }}>
      <h1>🚀 Dexeme</h1>
      <p>Automated Memecoin Index Fund on Solana</p>
      <WalletButton />
      <DepositForm />
    </div>
  )
}
