"use client"

import { Toaster } from "@/components/ui/toaster"

import AccountBalance from "./AccountBalance"
import { AccountProvider } from "./AccountContext"
import { SubstrateProvider } from "./SubstrateContext"

function MemeDapp({ children }) {
  return (
    <AccountProvider appName="polkadot-react-template">
      <SubstrateProvider providerUrl="ws://127.0.0.1:9944">
        <div className="relative flex min-h-screen flex-col">
          <div className="flex-1">{children}</div>
        </div>
        <Toaster />
      </SubstrateProvider>
    </AccountProvider>
  )
}

export default MemeDapp
