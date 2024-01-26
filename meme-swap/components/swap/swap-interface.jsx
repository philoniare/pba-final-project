"use client"

import React, { useEffect, useState } from "react"
import Image from "next/image"
import { ApiPromise, WsProvider } from "@polkadot/api"
import { Abi, ContractPromise } from "@polkadot/api-contract"
import {
  web3Accounts,
  web3Enable,
  web3FromSource,
} from "@polkadot/extension-dapp"

import { Button } from "@/components/ui/button"
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

const getApi = async () => {
  const provider = new WsProvider("ws://127.0.0.1:9944")
  const api = await ApiPromise.create({ provider })
  return api
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

export default function SwapInterface() {
  const [from, setFrom] = useState(TOKENS[0].symbol)
  const [to, setTo] = useState(TOKENS[1].symbol)
  const [wallet, setWallet] = useState(null)
  const [walletApi, setWalletApi] = useState(null)

  useEffect(() => {
    const startListener = async () => {
      const api = await getApi()
      setWalletApi(api)
      await api.query.system.events((events) => {
        events.forEach((record) => {
          const { event, phase } = record
          const types = event.typeDef
          console.log("event", record.method)
          if (
            event.section === "contracts" &&
            event.method === "ContractEmitted"
          ) {
            const [contractAddress, eventData] = event.data
          }
        })
      })
    }
    startListener()
  }, [])

  const swap = async () => {
    const injector = await web3FromSource(wallet.meta.source)
    walletApi.setSigner(injector.signer)
    const hash = await walletApi.tx.dex
      .swap(
        {
          gasLimit: 50000000,
        },
        1,
        2,
        100000000000
      )
      .signAndSend(wallet.address, (res) => {
        console.log("resInBlock", res.status.isInBlock)
        console.log("resIsFinal", res.status.isFinalized)
        console.log("res", res.status.toJSON())
      })
      .catch((err) => console.log(`😞 Transaction Failed: ${err.toString()}`))
    console.log("hash", hash)
  }

  const initWallet = async () => {
    const injectedExtensions = await web3Enable("MeMeSwap")
    if (injectedExtensions.length === 0) {
      return
    }

    const allAccounts = await web3Accounts()
    setWallet(allAccounts[0])
  }

  return (
    <main className="flex min-h-screen flex-col justify-start items-center p-24">
      <div className="z-10 max-w-5xl w-full items-center justify-between font-mono text-sm lg:flex">
        <p className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-[#e6007a] from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:from-inherit lg:static lg:w-auto lg:rounded-xl lg:border lg:p-4 lg:dark:bg-zinc-800/30">
          Swap your Favorite
          <code className="font-mono font-bold ml-2">Meme Tokens!</code>
        </p>
      </div>

      <div className="flex flex-row gap-10 mt-4">
        <div className="relative flex place-items-center before:absolute before:h-[300px] before:w-full sm:before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-full sm:after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#0141ff] after:dark:opacity-40 before:lg:h-[360px] z-[-1]">
          <Image
            className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
            src="https://camo.githubusercontent.com/6fd8a0eed38bca84442d764612c636cee4f3fcabc2cbb8a5e8920978233e6936/68747470733a2f2f692e6962622e636f2f4350636b305a522f44414c4c2d452d323032342d30312d32352d31362d31322d35392d44657369676e2d612d6c6f676f2d666f722d4d656d652d537761702d612d556e69737761702d7374796c652d646563656e7472616c697a65642d746f6b656e2d65786368616e67652e706e67"
            alt="Next.js Logo"
            width={180}
            height={37}
            priority
          />
        </div>

        <div className="flex flex-col">
          <div className="flex flex-col space-y-2 mb-4">
            <Label htmlFor="from-token">From</Label>
            <Select onValueChange={(value) => setFrom(value)}>
              <SelectTrigger className="w-[180px]">
                <SelectValue placeholder="From Token" />
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

          <div className="flex flex-col space-y-2 mb-4">
            <Label htmlFor="to-token">To</Label>
            <Select onValueChange={(value) => setTo(value)}>
              <SelectTrigger className="w-[180px]">
                <SelectValue placeholder="To Token" />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>Token</SelectLabel>
                  {TOKENS.filter((token) => token.symbol !== from).map(
                    (token) => (
                      <SelectItem key={token.symbol} value={token.symbol}>
                        {token.symbol}
                      </SelectItem>
                    )
                  )}
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>

          <div className="flex flex-col space-y-2 mb-4">
            <Label htmlFor="to-token">Amount</Label>
            <Input id="to-token" placeholder="Amount" />
          </div>

          <Button onClick={initWallet} className="w-full">
            Swap
          </Button>
        </div>
      </div>
    </main>
  )
}
