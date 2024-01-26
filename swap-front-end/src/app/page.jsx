'use client';
import React, {useEffect, useState} from "react";
import dynamic from "next/dynamic";

const PolkadotDapp = dynamic(
    () => import('@/components/MainApp'),
    { ssr: false }
);

export default function Home() {

  return (
    <PolkadotDapp />
  );
}
