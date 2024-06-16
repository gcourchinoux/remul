
from transformers import AutoTokenizer, MarianMTModel

model = MarianMTModel.from_pretrained('Helsinki-NLP/opus-mt-fr-en')

tokenizer = AutoTokenizer.from_pretrained('Helsinki-NLP/opus-mt-fr-en')


f = open('../README_FR.MD')

f = f.read()


str_ = f.split('.')



TMP = 0
f = open('texte_traduit.txt',"w")

for st in str_:
    print(st)
    batch = tokenizer([st], return_tensors="pt")
    generated_ids = model.generate(**batch)
    TMP = tokenizer.batch_decode(generated_ids, skip_special_tokens=True)[0]
    print(TMP)
    print(type(TMP))
    f.write(TMP+".")
    f.write('\n')




f.close()