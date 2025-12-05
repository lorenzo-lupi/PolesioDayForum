"use client";

import type { FC } from "react";
import { useState } from "react";
// Se usi NextAuth:
import { useSession, signIn, signOut } from "next-auth/react";

type Event = {
  id: number;
  title: string;
  time: string;
  description: string;
  place: string;
};

const EVENTS: Event[] = [
  {
    id: 1,
    title: "Arrivo, caffè e abbracci",
    time: "10:00 — 11:30",
    description:
      "Ci ritroviamo nel cortile di Polesio, due chiacchiere, un caffè e il tempo di salutare tutti con calma.",
    place: "Cortile di Polesio",
  },
  {
    id: 2,
    title: "Pranzo lungo da tavolata",
    time: "12:30 — 15:00",
    description:
      "Tavolata unica con tutto quello che ognuno porta: primi, secondi, dolci, racconti e risate.",
    place: "Sotto il pergolato",
  },
  {
    id: 3,
    title: "Giochi, passeggiata e foto di famiglia",
    time: "15:30 — 18:00",
    description:
      "Giochi per bimbi e grandi, passeggiata nei campi e foto di gruppo per ricordare l’edizione del giorno.",
    place: "Dintorni di Polesio",
  },
];

const PolesioDayLanding: FC = () => {
  // Se non hai ancora NextAuth configurato, puoi sostituire useSession con un mock semplice
  const { data: session } = useSession();
  const user = session?.user; // { name, email, image }

  const [subscriptions, setSubscriptions] = useState<number[]>([]);

  const toggleSubscription = (eventId: number) => {
    setSubscriptions((prev) =>
      prev.includes(eventId)
        ? prev.filter((id) => id !== eventId)
        : [...prev, eventId]
    );
  };

  return (
    <main className="min-h-screen bg-amber-50 text-stone-900">
      {/* sfondo decorativo chiaro */}
      <div className="pointer-events-none fixed inset-0 -z-10 overflow-hidden">
        <div className="absolute -left-24 -top-24 h-64 w-64 rounded-full bg-amber-200/40 blur-3xl" />
        <div className="absolute right-0 top-1/3 h-72 w-72 rounded-full bg-lime-200/40 blur-3xl" />
        <div className="absolute bottom-0 left-1/4 h-64 w-64 rounded-full bg-sky-200/40 blur-3xl" />
      </div>

      <div className="mx-auto flex min-h-screen max-w-5xl flex-col px-4 py-6 sm:px-6 lg:px-8">
        {/* Header */}
        <header className="flex items-center justify-between gap-4">
          <div className="flex items-center gap-3">
            <div className="flex h-11 w-11 items-center justify-center rounded-2xl border border-amber-400/70 bg-amber-100 text-lg font-semibold text-amber-900 shadow-sm">
              PD
            </div>
            <div className="leading-tight">
              <p className="text-xs uppercase tracking-[0.25em] text-amber-700/80">
                Polesio Day
              </p>
              <p className="text-sm text-stone-700">
                Riunione familiare semi-annuale
              </p>
            </div>
          </div>

          {/* Login / utente */}
          <div className="flex items-center gap-3">
            {user && (
              <div className="hidden flex-col items-end text-right text-xs text-stone-700 sm:flex">
                <span className="font-semibold text-stone-900">
                  {user.name ?? "Ospite"}
                </span>
                <span className="text-[11px] text-stone-500">
                  {user.email}
                </span>
              </div>
            )}

            {user && user.image && (
              <img
                src={user.image}
                alt={user.name ?? "Avatar"}
                referrerPolicy="no-referrer"
                className="hidden h-9 w-9 rounded-full border border-amber-400/70 bg-amber-100 object-cover sm:block"
              />
            )}

            {user ? (
              <button
                onClick={() => signOut()}
                className="rounded-full border border-stone-300 bg-white px-4 py-1.5 text-xs font-medium text-stone-800 shadow-sm hover:bg-stone-50 transition"
              >
                Esci
              </button>
            ) : (
              <button
                onClick={() => signIn("google")}
                className="inline-flex items-center gap-2 rounded-full border border-amber-500 bg-white px-4 py-1.5 text-xs font-medium text-amber-900 shadow-sm hover:bg-amber-50 transition"
              >
                {/* Mini “G” finta, solo estetica */}
                <span className="flex h-4 w-4 items-center justify-center rounded-full bg-white text-[9px] font-bold text-blue-600 border border-stone-300">
                  G
                </span>
                Accedi con Google
              </button>
            )}
          </div>
        </header>

        {/* Hero */}
        <section className="mt-10 flex flex-1 flex-col items-center justify-center gap-10 py-6 text-center">
          <div className="max-w-2xl space-y-4">
            <p className="inline-flex items-center gap-2 rounded-full border border-amber-300 bg-amber-100/80 px-4 py-1 text-[11px] font-medium uppercase tracking-[0.25em] text-amber-700">
              <span className="h-1.5 w-1.5 rounded-full bg-emerald-500" />
              Polesio Day • prossima edizione
            </p>

            <h1 className="text-balance text-4xl font-semibold tracking-tight text-stone-900 sm:text-5xl lg:text-6xl">
              La giornata di famiglia a{" "}
              <span className="bg-gradient-to-r from-amber-700 via-orange-600 to-lime-700 bg-clip-text text-transparent">
                Polesio
              </span>
            </h1>

            <p className="text-balance text-sm text-stone-700 sm:text-base">
              Due volte l&apos;anno la famiglia allargata si ritrova a Polesio
              per una giornata semplice, fatta di tavolate, storie, bambini che
              corrono e zii che parlano troppo. Come un pranzo di paese, ma
              solo con i nostri.
            </p>
          </div>

          {/* Info rapide */}
          <div className="grid w-full max-w-3xl gap-4 sm:grid-cols-3">
            <div className="rounded-2xl border border-amber-200 bg-white/80 p-4 text-left shadow-sm">
              <p className="text-[11px] font-semibold uppercase tracking-[0.25em] text-stone-500">
                Prossima data
              </p>
              <p className="mt-2 text-lg font-semibold text-amber-800">
                01 Giugno 2026
              </p>
              <p className="text-xs text-stone-500">Polesio • dalle 10:00</p>
            </div>

            <div className="rounded-2xl border border-amber-200 bg-white/80 p-4 text-left shadow-sm">
              <p className="text-[11px] font-semibold uppercase tracking-[0.25em] text-stone-500">
                Atmosfera
              </p>
              <p className="mt-2 text-sm text-stone-900">
                Intima, casalinga, da cortile e pergolato. Vestiti comodi,
                niente formalità.
              </p>
            </div>

            <div className="rounded-2xl border border-amber-200 bg-white/80 p-4 text-left shadow-sm">
              <p className="text-[11px] font-semibold uppercase tracking-[0.25em] text-stone-500">
                Familiari attesi
              </p>
              <p className="mt-2 text-3xl font-bold text-amber-700">~ 40</p>
              <p className="text-xs text-stone-500">Tra cugini, zii e nonni</p>
            </div>
          </div>
        </section>

        {/* Eventi & iscrizioni */}
        <section className="mt-4 grid gap-6 sm:grid-cols-[1.1fr,0.9fr]">
          {/* Eventi */}
          <div className="rounded-3xl border border-amber-200 bg-white/90 p-6 sm:p-7">
            <div className="flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between">
              <div>
                <h2 className="text-lg font-semibold text-stone-900 sm:text-xl">
                  Eventi del Polesio Day
                </h2>
                <p className="mt-1 text-sm text-stone-600">
                  L&apos;amministratore può modificare questi momenti
                  dall&apos;area di gestione (per ora sono esempi).
                </p>
              </div>
              <p className="text-[11px] uppercase tracking-[0.25em] text-amber-700 mt-1">
                Iscrizione per slot
              </p>
            </div>

            <div className="mt-5 space-y-4">
              {EVENTS.map((event) => {
                const isSubscribed = subscriptions.includes(event.id);
                return (
                  <div
                    key={event.id}
                    className="rounded-2xl border border-amber-100 bg-amber-50/70 p-4 text-left shadow-sm"
                  >
                    <div className="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
                      <div>
                        <p className="text-[11px] font-semibold uppercase tracking-[0.25em] text-amber-700">
                          {event.time}
                        </p>
                        <h3 className="mt-1 text-sm font-semibold text-stone-900">
                          {event.title}
                        </h3>
                        <p className="mt-1 text-xs text-stone-600">
                          {event.description}
                        </p>
                        <p className="mt-2 text-xs font-medium text-stone-700">
                          📍 {event.place}
                        </p>
                      </div>

                      <div className="flex flex-col items-start gap-2 sm:items-end">
                        {user ? (
                          <>
                            <button
                              onClick={() => toggleSubscription(event.id)}
                              className={`inline-flex items-center justify-center rounded-full px-4 py-1.5 text-xs font-medium shadow-sm transition ${
                                isSubscribed
                                  ? "bg-emerald-200 text-emerald-900 border border-emerald-400 hover:bg-emerald-100"
                                  : "bg-white text-amber-900 border border-amber-300 hover:bg-amber-50"
                              }`}
                            >
                              {isSubscribed ? "Sei iscritto" : "Iscriviti a questo momento"}
                            </button>
                            <p className="text-[11px] text-stone-500">
                              L&apos;iscrizione è solo indicativa, per
                              aiutarci a capire chi c&apos;è in ogni momento.
                            </p>
                          </>
                        ) : (
                          <>
                            <button
                              disabled
                              className="inline-flex items-center justify-center rounded-full border border-stone-300 bg-white px-4 py-1.5 text-xs font-medium text-stone-400 shadow-sm"
                            >
                              Accedi con Google per iscriverti
                            </button>
                            <p className="text-[11px] text-stone-500">
                              Prima fai login con Google in alto a destra.
                            </p>
                          </>
                        )}
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>

          {/* Come arrivare */}
          <div className="rounded-3xl border border-amber-200 bg-white/90 p-6 sm:p-7">
            <h2 className="text-lg font-semibold text-stone-900 sm:text-xl">
              Come arrivare a Polesio
            </h2>
            <p className="mt-2 text-sm text-stone-700">
              L&apos;indirizzo preciso gira nel gruppo di famiglia. Polesio è a
              pochi minuti in auto da{" "}
              <span className="font-semibold text-amber-800">
                [città di riferimento]
              </span>
              , con parcheggio libero nei dintorni.
            </p>

            <ul className="mt-4 space-y-2 text-sm text-stone-700">
              <li>• Se arrivi in treno, organizziamo i passaggi in auto.</li>
              <li>• Se porti qualcosa da mangiare, meglio ancora.</li>
              <li>• In caso di pioggia ci stringiamo dentro casa.</li>
            </ul>

            <div className="mt-5 flex h-40 w-full items-center justify-center rounded-2xl border border-dashed border-amber-300 bg-amber-50/70 px-4 text-center text-xs text-stone-500">
              Qui puoi inserire in futuro una mappa (Google Maps / Leaflet)
              con il punto esatto di Polesio.
            </div>
          </div>
        </section>

        {/* Footer */}
        <footer className="mt-8 border-t border-amber-200/80 py-5 text-center text-[11px] text-stone-500">
          Polesio Day • Riunione familiare semi-annuale •{" "}
          <span className="font-semibold text-amber-800">Polesio</span>
        </footer>
      </div>
    </main>
  );
};

export default PolesioDayLanding;
