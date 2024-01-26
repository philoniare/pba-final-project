"use client"

import React, { useState } from "react"
import { web3FromAddress } from "@polkadot/extension-dapp"

import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
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
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useToast } from "@/components/ui/use-toast"

import { useAccount } from "./AccountContext"
import { useSubstrate } from "./SubstrateContext"

const ASSET_IDS = {
  "DOGE": 5,
  "SHIP": 6,
  "PEPE": 7,
  "BONK": 8,
}

const TOKENS = [
  {
    symbol: "DOGE",
  },
  {
    symbol: "SHIB",
  },
  {
    symbol: "PEPE",
  },
  {
    symbol: "BONK",
  },
]

const Swap = () => {
  const { api } = useSubstrate()
  const { selectedAccount } = useAccount()
  const [status, setStatus] = useState("")
  const [remark, setRemark] = useState("")
  const [from, setFrom] = useState(TOKENS[0].symbol)
  const [to, setTo] = useState(TOKENS[1].symbol)
  const [amount, setAmount] = useState("0")
  const { toast } = useToast()

  const handleRemarkChange = (event) => {
    setRemark(event.target.value)
  }

  const handleCreatePool = async (e) => {
    e.preventDefault()
    try {
      if (api && selectedAccount) {
        const { address } = selectedAccount

        // Use the injected account for signing
        const injector = await web3FromAddress(address)
        // Need to manually specify the first param (asset_id)
        // Ideally, would create a special indexer for keeping
        // track of existing asset_ids
        const unsubscribe = await api.tx.dex
          .mint(9997, ASSET_IDS[from], ASSET_IDS[to], parseInt(amount), parseInt(amount))
          .signAndSend(address, { signer: injector.signer }, ({ status }) => {
            setStatus(`Current status is ${status}`)

            if (status.isInBlock) {
              toast({
                title: "Transaction included at blockhash",
                description: status.asInBlock,
              })
            } else if (status.isFinalized) {
              toast({
                title: "Transaction finalized at blockHash",
                description: status.asFinalized,
              })
              unsubscribe()
            }
          })

        // Clear the remark field after submission
        setRemark("")
      }
    } catch (error) {
      console.error("Error submitting transaction:", error)
      setStatus(`Error: ${error.message}`)
    }
  }

  const handleSwap = async (e) => {
    e.preventDefault()
    try {
      if (api && selectedAccount) {
        const { address } = selectedAccount

        // Use the injected account for signing
        const injector = await web3FromAddress(address)
        const unsubscribe = await api.tx.dex
          .swap(ASSET_IDS[from], ASSET_IDS[to], parseInt(amount))
          .signAndSend(address, { signer: injector.signer }, ({ status }) => {
            setStatus(`Current status is ${status}`)

            if (status.isInBlock) {
              toast({
                title: "Transaction included at blockhash",
                description: status.asInBlock,
              })
            } else if (status.isFinalized) {
              toast({
                title: "Transaction finalized at blockHash",
                description: status.asFinalized,
              })
              unsubscribe()
            }
          })

        // Clear the remark field after submission
        setRemark("")
      }
    } catch (error) {
      console.error("Error submitting transaction:", error)
      setStatus(`Error: ${error.message}`)
    }
  }

  // Disable the component if there is no API or no selected account
  if (!api || !selectedAccount) {
    return (
      <div className={"w-full flex justify-center"}>
        <p>Please connect to the Polkadot extension and select an account.</p>
      </div>
    )
  }

  return (
    <div className="flex w-full flex-col items-center justify-center">
      <div style={{ width: "50%" }}>
        <Tabs
          defaultValue="pool"
          className="w-[400px]"
          style={{ width: "400px" }}
        >
          <TabsList
            className="flex w-full flex-row justify-around"
            style={{ justifyContent: "space-around" }}
          >
            <TabsTrigger value="pool" style={{ width: "190px" }}>
              Pool
            </TabsTrigger>
            <TabsTrigger style={{ width: "190px" }} value="swap">
              Swap
            </TabsTrigger>
          </TabsList>
          <TabsContent value="pool">
            <Card className="w-[350px]">
              <CardHeader>
                <CardTitle>Create</CardTitle>
                <CardDescription>
                  A pool of your favorite meme tokens.
                </CardDescription>
              </CardHeader>
              <CardContent>
                <form>
                  <div className="flex flex-col">
                    <div
                      className="mb-4 flex flex-col space-y-2"
                      style={{ marginBottom: "8px" }}
                    >
                      <Select onValueChange={(value) => setFrom(value)}>
                        <SelectTrigger className="w-[180px]">
                          <SelectValue placeholder="Token A" />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectGroup>
                            <SelectLabel>Token</SelectLabel>
                            {TOKENS.map((token) => (
                              <SelectItem
                                key={token.symbol}
                                value={token.symbol}
                              >
                                {token.symbol}
                              </SelectItem>
                            ))}
                          </SelectGroup>
                        </SelectContent>
                      </Select>
                    </div>

                    <div
                      className="flex flex-col space-y-2 mb-4"
                      style={{ marginBottom: "8px" }}
                    >
                      <Select onValueChange={(value) => setTo(value)}>
                        <SelectTrigger className="w-[180px]">
                          <SelectValue placeholder="Token B" />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectGroup>
                            <SelectLabel>Token</SelectLabel>
                            {TOKENS.filter(
                              (token) => token.symbol !== from
                            ).map((token) => (
                              <SelectItem
                                key={token.symbol}
                                value={token.symbol}
                              >
                                {token.symbol}
                              </SelectItem>
                            ))}
                          </SelectGroup>
                        </SelectContent>
                      </Select>
                    </div>

                    <div
                      className="flex flex-col space-y-2 mb-4"
                      style={{ marginBottom: "8px" }}
                    >
                      <Input
                        id="to-token"
                        value={amount}
                        onChange={(e) => {
                          setAmount(e.target.value)
                        }}
                        placeholder="Amount"
                      />
                    </div>

                    <Button onClick={handleCreatePool} className="w-full">
                      Create Liquidity Pool
                    </Button>
                  </div>
                </form>
              </CardContent>
            </Card>
          </TabsContent>
          <TabsContent value="swap">
            <Card className="w-[350px]">
              <CardHeader>
                <CardTitle>Swap</CardTitle>
                <CardDescription>Your favorite meme tokens</CardDescription>
              </CardHeader>
              <CardContent>
                <form>
                  <div
                    className="mb-4 flex flex-col space-y-2"
                    style={{ marginBottom: "8px" }}
                  >
                    <Select onValueChange={(value) => setFrom(value)}>
                      <SelectTrigger className="w-[180px]">
                        <SelectValue placeholder="From" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectGroup>
                          <SelectLabel>Token</SelectLabel>
                          {TOKENS.map((token) => (
                            <SelectItem key={token.symbol} value={token.symbol}>
                              {token.symbol}
                            </SelectItem>
                          ))}
                        </SelectGroup>
                      </SelectContent>
                    </Select>
                  </div>

                  <div
                    className="flex flex-col space-y-2 mb-4"
                    style={{ marginBottom: "8px" }}
                  >
                    <Select onValueChange={(value) => setTo(value)}>
                      <SelectTrigger className="w-[180px]">
                        <SelectValue placeholder="To" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectGroup>
                          <SelectLabel>Token</SelectLabel>
                          {TOKENS.filter((token) => token.symbol !== from).map(
                            (token) => (
                              <SelectItem
                                key={token.symbol}
                                value={token.symbol}
                              >
                                {token.symbol}
                              </SelectItem>
                            )
                          )}
                        </SelectGroup>
                      </SelectContent>
                    </Select>
                  </div>

                  <div
                    className="flex flex-col space-y-2 mb-4"
                    style={{ marginBottom: "8px" }}
                  >
                    <Input id="to-token" placeholder="Amount" />
                  </div>

                  <Button onClick={handleSwap} className="w-full">
                    Swap
                  </Button>
                </form>
              </CardContent>
            </Card>
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}

export default Swap
