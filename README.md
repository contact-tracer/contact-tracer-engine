# CTE: contact-tracer-engine

Privacy-preserving backend library for COVID-19 contact tracing apps.

Our goal is to make a back-end library that is so useful to developers of contact tracing apps, that it becomes the obvious and de-facto choice. Especially targeted for developers who are indifferent about privacy.

## Preface

Contact tracing is a key part of managing epidemics [[1](https://www.who.int/features/qa/contact-tracing/en/)]. It is a labour-intensive process. The scale of the COVID-19 pandemic means auto-recording potential contacts using smartphones is an obvious contribution to scaling up contact tracing.

Our concern is that simplistic designs will compromise privacy, irrevocably [[2](https://www.top10vpn.com/news/surveillance/covid-19-digital-rights-tracker/)]. It will be extremely difficult to add privacy later on. Just like the technical and adoption difficulties found trying to retroactively add security to FTP and http protocols, and especially SMTP - email - which is so bad it's still not fixed 30 years later.

Privacy or public health is a false dichotomy. This engine is to make it easier for everyone to have both.

## Storyboard for App Developers

Contact-tracing-engine (CTE) is a library that supplies seemingly unrelated ids for each contact and, for users marked as COVID-19 positive, it manages notifications between those contacts.

The contact tracing app developer uses their own definition of what a 'contact' is, for example proximity and duration. They also use their method of choice to identify contact, for example GPS, BLE Bluetooth Low Energy, QR code, manual check-in, etc. The library stores contact pairs, each pair comprising their id and the other party's contact id. The library also manages broadcasting to, and monitoring for notifications from potential contacts.

Using the library saves the developer time and effort on id generation, use, management, privacy-preserving notifications and privacy-preserving listening. It requires no centralised server.

1. A contact tracing app generates a new secret, and gives it to the user to write down as backup. The secret is expressed as a string of words, making it easy to write by hand. The library is initialised with this secret.
   1. If the user dies and is identified as COVID-19 positive at post-mortem, next of kin can use the backup to notify all potential contacts. They do not need access to the deceased's phone.
1. When a contact is made, the app gets a contact id from the library, and negotiates exchanging it with the other party and receiving their contact id.
   1. A contact id is only ever used once. This means that a malicious app cannot 'follow' a contact through time, because the id is never re-used.
   1. The ids are seemingly random, but in fact are deterministically derived from the secret mentioned in the first bullet. The secret deterministically generates private keys, and the contact id is the hash of the corresponding public key. This concept is taken from cryptocurrencies, where the ids are known as Hierarchical Deterministic or HD addresses.
1. The app tells the library about the contact, so the library can record the pair. App developers must not record contact pairs within their app, or pass those pairs to any server.
1. If a user marks themselves as COVID-19 positive, the app makes a call to the library. The library crafts a special transaction, which includes a coded message, to all the ids that the app has made contact with within a given timeframe. The library makes the transaction indistinguishable from many other transactions on that cryptocurrency network.
1. The other contacts' app sees the transaction and informs their user of the new contact status. They should advise what the contact status means, actions that should be followed including testing and self-isolation or quarantine. The local protocol for the other contacts may be different to our original contact - for example they may be in different cities, regions or countries.
1. The other contacts' app in turn notifies their respective contacts, again according local rules. They also communicate actions taken and updates to their COVID-19 status to their contacts, which includes the contact(s) of our originating app.


## References

1. [WHO Contact Tracing](https://www.who.int/features/qa/contact-tracing/en/)
1. [COVID-19 Digital Rights Tracker](https://www.top10vpn.com/news/surveillance/covid-19-digital-rights-tracker/)
1. EFF COVID-19 and Digital Rights
1. Contact Tracing Mobile Apps for COVID-19: Privacy Considerations and Related Trade-offs
