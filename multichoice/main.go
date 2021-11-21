package main

import (
	"log"
	"os"
	"strings"

	"github.com/lxn/walk"
	//lint:ignore ST1001 standard behavior lxn/walk
	. "github.com/lxn/walk/declarative"
)

type MyMainWindow struct {
	*walk.MainWindow
}

var inputChoices []string
var outputChoices []string

func main() {
	log.SetFlags(log.LstdFlags | log.Lshortfile)
	argsWithoutProg := os.Args[1:]
	if len(argsWithoutProg) != 3 {
		log.Println("Usage: multichoice Title Prompt Choices;here")
		os.Exit(1)
	}
	title := argsWithoutProg[0]
	prompt := argsWithoutProg[1]
	inputChoices = strings.Split(argsWithoutProg[2], ";")

	mw := new(MyMainWindow)
	if err := (MainWindow{
		AssignTo: &mw.MainWindow,
		Title:    title,
		Size: Size{
			Width:  30,
			Height: 20,
		},
		Layout: VBox{},
		Children: []Widget{
			Label{
				Text: prompt,
			},
			Composite{
				Alignment: AlignHNearVNear,
				Layout:    VBox{},
				Children:  CheckBoxList(inputChoices),
			},
			Composite{
				Layout: HBox{},
				Children: []Widget{
					PushButton{
						Text: "Ok",
						OnClicked: func() {
							println(strings.Join(outputChoices, ";"))
							os.Exit(0)
						},
					},
					PushButton{
						Text: "Cancel",
						OnClicked: func() {
							os.Exit(0)
						},
					},
				},
			},
		},
	}.Create()); err != nil {
		log.Fatal(err)
	}

	mw.Run()
}

func CheckBoxList(choices []string) []Widget {
	bs := make([]*walk.CheckBox, len(choices))
	children := []Widget{}
	for i, name := range choices {
		bs[i] = new(walk.CheckBox)
		indexName := name
		children = append(children, CheckBox{
			AssignTo: &bs[i],
			Text:     name,
			OnClicked: func() {
				if i := contains(indexName, outputChoices); i != -1 {
					outputChoices = remove(outputChoices, i)
				} else {
					outputChoices = append(outputChoices, indexName)
				}
			},
		})
	}
	return children
}

func contains(target string, array []string) int {
	for index, value := range array {
		if value == target {
			return index
		}
	}
	return -1
}

func remove(s []string, i int) []string {
	s[i] = s[len(s)-1]
	return s[:len(s)-1]
}
