import Swap from "@/components/swap/Swap"

export default function IndexPage() {
  return (
    <section className="container grid items-center gap-6 pb-8 pt-6 md:py-10">
      <div className="flex max-w-[980px] flex-col items-start gap-2 justify-center items-center">
        <h1 className="text-center text-3xl font-extrabold leading-tight tracking-tighter md:text-4xl">
          A new playground for meme coin enthusiasts. <br /> Happy Swapping!
        </h1>
      </div>
      <div className="flex gap-4">
        <Swap />
      </div>
    </section>
  )
}
