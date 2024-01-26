import { SubstrateProvider } from './SubstrateContext';
import { AccountProvider } from './AccountContext';
import AccountBalance from './AccountBalance';
// import Remark from './Remark';

function App() {
    return (
        <div>
            <h1>Polkadot React Template</h1>
            <AccountProvider appName="polkadot-react-template">
                <SubstrateProvider providerUrl="wss://rpc.polkadot.io">
                    <h2>MeMeSwap</h2>
                    <AccountBalance />
                    {/*<Remark />*/}
                </SubstrateProvider>
            </AccountProvider>
        </div>
    );
}

export default App;