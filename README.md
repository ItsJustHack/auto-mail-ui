# Envoi automatique de Mail

Cette application a été développée pour envoyer automatiquement des mails (ici aux entreprises) avec des templates prédéfinis.

# Configuration

Lors de la première installation, l'utilisateur doit changer deux fichiers, ces deux fichiers se situent dans le dossier d'installation sur Windows (C:\Program Files\auto-mail-forum\config par défaut)  : 

- config.toml
- credentials.toml

Ces deux fichiers doivent simplement être remplacés avec vos identifiants / noms / prénoms, ...

# Ajoutez des templates / Pièces jointes

De même vous pouvez, si vous le voulez, rajouter davantage de templates avec les pièces jointes attachées qui doivent être dans le dossier plaquettes.

Pour cela, dans le dossier d'installation du programme vous trouverez différents différents dossiers : 
- mails
- plaquettes
- config
- icons

## Dossier mails

Ce dossier contient toutes les templates pour les mails, vous pouvez en rajouter autant que vous le désirez à condition de les indiquer dans le fichier de configuration, voir plus bas.

## Dossier plaquettes 

Ce dossier contient toutes les plaquettes pour les mails, autrement dit toutes les pièces jointes que vous voudriez attacher, à priori ces dernières sont uniquement des PDFs pour le moment, je pourrais rajouter d'autres types si besoin dans le futur

## Dossier config

Ce dossier contient tous les fichiers de configurations, vous avez normalement déjà modifiés le fichier 'config.toml' et 'credentials.toml', le dernier correspond à donner les templates au système. Le fichier 'email_type.toml' correpond donc à toutes les templates que vous voudriez rajouter. Si vous voulez en rajouter il faut suivre la même syntaxe que pour les deux prédéfinis, i.e. : 

```toml
[NouveauTemplate]
mail_path = "./mails/nouveau_template"
objet = "Objet du nouveau template"
attachements = [
  "./plaquettes/attachements1",
  "./plaquettes/attachements1",
]
```

