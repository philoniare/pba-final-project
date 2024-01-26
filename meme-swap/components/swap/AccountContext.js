import React, { createContext, useContext, useState } from "react"
import Link from "next/link"
import { web3AccountsSubscribe, web3Enable } from "@polkadot/extension-dapp"

import { siteConfig } from "@/config/site"
import { Label } from "@/components/ui/label"
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Icons } from "@/components/icons"
import { MainNav } from "@/components/main-nav"
import { ThemeToggle } from "@/components/theme-toggle"

import { Button, buttonVariants } from "../ui/button"

const AccountContext = createContext()

const AccountProvider = ({ appName, children }) => {
  const [selectedAccount, setSelectedAccount] = useState(null)
  const [accounts, setAccounts] = useState([])
  const [isConnected, setIsConnected] = useState(false)

  const connectAccounts = async () => {
    try {
      await web3Enable(appName)
      const unsubscribe = await web3AccountsSubscribe((injectedAccounts) => {
        setAccounts(injectedAccounts)
        if (injectedAccounts.length > 0 && !selectedAccount) {
          // Set the first account as the selected account initially
          setSelectedAccount(injectedAccounts[0])
        }
      })

      setIsConnected(true)
      return () => unsubscribe()
    } catch (error) {
      console.error("Error fetching accounts:", error)
    }
  }

  const handleAccountChange = (account) => {
    setSelectedAccount(account)
  }

  const fetchShortAddress = (address) => {
    return address.slice(0, 6) + "..." + address.slice(-4)
  }

  return (
    <AccountContext.Provider value={{ selectedAccount, setSelectedAccount }}>
      <header
        className="bg-background sticky top-0 z-40 w-full border-b"
        style={{ marginTop: "20px" }}
      >
        <div className="container flex h-16 items-center space-x-4 sm:justify-between sm:space-x-0">
          <MainNav items={siteConfig.mainNav} />
          <div
            className="flex flex-1 items-center justify-end space-x-4"
            style={{ justifyContent: "end" }}
          >
            <nav className="flex items-center space-x-1">
              <div>
                {!isConnected ? (
                  <Button onClick={connectAccounts} className="w-[300px]">
                    Connect
                  </Button>
                ) : accounts.length > 0 ? (
                  <div className="flex flex-col space-y-2 mb-4">
                    <Select
                      value={selectedAccount?.address}
                      onValueChange={(account) => handleAccountChange(account)}
                    >
                      <SelectTrigger className="w-[180px]">
                        <SelectValue placeholder="Wallet" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectGroup>
                          <SelectLabel>Connected wallets</SelectLabel>
                          {accounts.map((account) => (
                            <SelectItem
                              key={account.address}
                              value={account.address}
                            >
                              {fetchShortAddress(account.address)}
                            </SelectItem>
                          ))}
                        </SelectGroup>
                      </SelectContent>
                    </Select>
                  </div>
                ) : (
                  <p>No accounts found</p>
                )}
              </div>
            </nav>
          </div>
        </div>
      </header>
      {children}
    </AccountContext.Provider>
  )
}

const useAccount = () => {
  const context = useContext(AccountContext)
  if (!context) {
    throw new Error("useAccount must be used within an AccountProvider")
  }
  return context
}

export { AccountProvider, useAccount }
