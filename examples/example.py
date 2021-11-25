from sonic import IngestChannel, SearchChannel, ControlChannel


def main():
    ingestcl = IngestChannel("localhost:1491", "SecretPassword")
    print(ingestcl.ping())
    ingestcl.push("wiki", "articles", "article-1", "for the love of god hell")
    ingestcl.push("wiki", "articles", "article-2", "for the love of satan heaven")
    ingestcl.push("wiki", "articles", "article-3", "for the love of lorde hello")
    ingestcl.push("wiki", "articles", "article-4", "for the god of loaf helmet")

    querycl = SearchChannel("localhost:1491", "SecretPassword")
    print(querycl.ping())
    print(querycl.query("wiki", "articles", "for"))
    print(querycl.query("wiki", "articles", "love"))
    print(querycl.suggest("wiki", "articles", "hell"))

    controlcl = ControlChannel("localhost:1491", "SecretPassword")
    print(controlcl.ping())
    print(controlcl.consolidate())


if __name__ == '__main__':
    main()
